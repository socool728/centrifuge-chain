use super::*;

/// Used where the error comes from other pallet impl. unknown from the tests
const PRICE_ID_NO_FOUND: DispatchError = DispatchError::Other("Price ID not found");

fn config_mocks(withdraw_amount: Balance) {
	MockPools::mock_withdraw(move |pool_id, to, amount| {
		assert_eq!(to, BORROWER);
		assert_eq!(pool_id, POOL_A);
		assert_eq!(withdraw_amount, amount);
		Ok(())
	});
	MockPrices::mock_get(|id| match *id {
		REGISTER_PRICE_ID => Ok((PRICE_VALUE, BLOCK_TIME.as_secs())),
		_ => Err(PRICE_ID_NO_FOUND),
	});
	MockPrices::mock_register_id(|id, pool_id| {
		assert_eq!(*pool_id, POOL_A);
		match *id {
			REGISTER_PRICE_ID => Ok(()),
			_ => Err(PRICE_ID_NO_FOUND),
		}
	});
}

#[test]
fn with_wrong_loan_id() {
	new_test_ext().execute_with(|| {
		config_mocks(COLLATERAL_VALUE);

		assert_noop!(
			Loans::borrow(RuntimeOrigin::signed(BORROWER), POOL_A, 0, COLLATERAL_VALUE),
			Error::<Runtime>::LoanNotActiveOrNotFound
		);
	});
}

#[test]
fn from_other_borrower() {
	new_test_ext().execute_with(|| {
		let loan_id = util::create_loan(util::base_internal_loan());

		config_mocks(COLLATERAL_VALUE);

		assert_noop!(
			Loans::borrow(
				RuntimeOrigin::signed(OTHER_BORROWER),
				POOL_A,
				loan_id,
				COLLATERAL_VALUE
			),
			Error::<Runtime>::NotLoanBorrower
		);
	});
}

#[test]
fn with_restriction_no_written_off() {
	new_test_ext().execute_with(|| {
		let loan_id = util::create_loan(util::base_internal_loan());

		config_mocks(COLLATERAL_VALUE / 2);
		assert_ok!(Loans::borrow(
			RuntimeOrigin::signed(BORROWER),
			POOL_A,
			loan_id,
			COLLATERAL_VALUE / 2
		));

		advance_time(YEAR + DAY);
		util::write_off_loan(loan_id);

		assert_noop!(
			Loans::borrow(
				RuntimeOrigin::signed(BORROWER),
				POOL_A,
				loan_id,
				COLLATERAL_VALUE / 2
			),
			Error::<Runtime>::from(BorrowLoanError::Restriction)
		);
	});
}

#[test]
fn with_restriction_full_once() {
	new_test_ext().execute_with(|| {
		let loan_id = util::create_loan(LoanInfo {
			restrictions: LoanRestrictions {
				borrows: BorrowRestrictions::FullOnce,
				repayments: RepayRestrictions::FullOnce,
			},
			..util::base_internal_loan()
		});

		config_mocks(COLLATERAL_VALUE / 2);
		assert_noop!(
			Loans::borrow(
				RuntimeOrigin::signed(BORROWER),
				POOL_A,
				loan_id,
				COLLATERAL_VALUE / 2 // Must be full value
			),
			Error::<Runtime>::from(BorrowLoanError::Restriction)
		);

		config_mocks(COLLATERAL_VALUE);
		assert_ok!(Loans::borrow(
			RuntimeOrigin::signed(BORROWER),
			POOL_A,
			loan_id,
			COLLATERAL_VALUE
		));

		// Borrow was already done
		assert_noop!(
			Loans::borrow(RuntimeOrigin::signed(BORROWER), POOL_A, loan_id, 0),
			Error::<Runtime>::from(BorrowLoanError::Restriction)
		);
	});
}

#[test]
fn with_maturity_passed() {
	new_test_ext().execute_with(|| {
		let loan_id = util::create_loan(util::base_internal_loan());

		advance_time(YEAR);

		config_mocks(COLLATERAL_VALUE);
		assert_noop!(
			Loans::borrow(
				RuntimeOrigin::signed(BORROWER),
				POOL_A,
				loan_id,
				COLLATERAL_VALUE
			),
			Error::<Runtime>::from(BorrowLoanError::MaturityDatePassed)
		);
	});
}

#[test]
fn with_wrong_big_amount_internal_pricing() {
	let borrow_inputs = [
		(COLLATERAL_VALUE + 1, util::total_borrowed_rate(1.0)),
		(COLLATERAL_VALUE / 2 + 1, util::total_borrowed_rate(0.5)),
		(1, util::total_borrowed_rate(0.0)),
		(COLLATERAL_VALUE + 1, util::outstanding_debt_rate(1.0)),
		(COLLATERAL_VALUE / 2 + 1, util::outstanding_debt_rate(0.5)),
		(1, util::outstanding_debt_rate(0.0)),
	];

	for (amount, max_borrow_amount) in borrow_inputs {
		new_test_ext().execute_with(|| {
			let loan_id = util::create_loan(LoanInfo {
				pricing: Pricing::Internal(InternalPricing {
					max_borrow_amount,
					..util::base_internal_pricing()
				}),
				..util::base_internal_loan()
			});

			config_mocks(amount);
			assert_noop!(
				Loans::borrow(RuntimeOrigin::signed(BORROWER), POOL_A, loan_id, amount),
				Error::<Runtime>::from(BorrowLoanError::MaxAmountExceeded)
			);
		});
	}
}

#[test]
fn with_correct_amount_internal_pricing() {
	let borrow_inputs = [
		(COLLATERAL_VALUE, util::total_borrowed_rate(1.0)),
		(COLLATERAL_VALUE / 2, util::total_borrowed_rate(0.5)),
		(0, util::total_borrowed_rate(0.0)),
		(COLLATERAL_VALUE, util::outstanding_debt_rate(1.0)),
		(COLLATERAL_VALUE / 2, util::outstanding_debt_rate(0.5)),
		(0, util::outstanding_debt_rate(0.0)),
	];

	for (amount, max_borrow_amount) in borrow_inputs {
		new_test_ext().execute_with(|| {
			let loan_id = util::create_loan(LoanInfo {
				pricing: Pricing::Internal(InternalPricing {
					max_borrow_amount,
					..util::base_internal_pricing()
				}),
				..util::base_internal_loan()
			});

			config_mocks(amount);
			assert_ok!(Loans::borrow(
				RuntimeOrigin::signed(BORROWER),
				POOL_A,
				loan_id,
				amount
			));
			assert_eq!(amount, util::current_loan_debt(loan_id));
		});
	}
}

#[test]
fn with_unregister_price_id() {
	new_test_ext().execute_with(|| {
		let loan = LoanInfo {
			pricing: Pricing::External(ExternalPricing {
				price_id: UNREGISTER_PRICE_ID,
				max_borrow_amount: ExtMaxBorrowAmount::Quantity(QUANTITY),
			}),
			..util::base_external_loan()
		};

		let loan_id = util::create_loan(loan);

		let amount = PRICE_VALUE.saturating_mul_int(QUANTITY);
		config_mocks(amount);

		assert_noop!(
			Loans::borrow(RuntimeOrigin::signed(BORROWER), POOL_A, loan_id, amount),
			PRICE_ID_NO_FOUND
		);
	});
}

#[test]
fn with_wrong_big_amount_external_pricing() {
	new_test_ext().execute_with(|| {
		let loan_id = util::create_loan(util::base_external_loan());

		let amount = PRICE_VALUE.saturating_mul_int(QUANTITY) + 1;
		config_mocks(amount);

		assert_noop!(
			Loans::borrow(RuntimeOrigin::signed(BORROWER), POOL_A, loan_id, amount),
			Error::<Runtime>::from(BorrowLoanError::MaxAmountExceeded)
		);
	});
}

#[test]
fn with_wrong_quantity_amount_external_pricing() {
	new_test_ext().execute_with(|| {
		let loan_id = util::create_loan(util::base_external_loan());

		// It's not multiple of PRICE_VALUE
		let amount = PRICE_VALUE.saturating_mul_int(QUANTITY) - 1;
		config_mocks(amount);

		assert_noop!(
			Loans::borrow(RuntimeOrigin::signed(BORROWER), POOL_A, loan_id, amount),
			Error::<Runtime>::AmountNotMultipleOfPrice
		);
	});
}

#[test]
fn with_correct_amount_external_pricing() {
	new_test_ext().execute_with(|| {
		let loan_id = util::create_loan(util::base_external_loan());

		let amount = PRICE_VALUE.saturating_mul_int(QUANTITY);
		config_mocks(amount);

		assert_ok!(Loans::borrow(
			RuntimeOrigin::signed(BORROWER),
			POOL_A,
			loan_id,
			amount
		),);
	});
}

#[test]
fn with_unlimited_amount_external_pricing() {
	new_test_ext().execute_with(|| {
		let loan = LoanInfo {
			pricing: Pricing::External(ExternalPricing {
				price_id: REGISTER_PRICE_ID,
				max_borrow_amount: ExtMaxBorrowAmount::NoLimit,
			}),
			..util::base_external_loan()
		};

		let loan_id = util::create_loan(loan);

		let amount = PRICE_VALUE.saturating_mul_int(2 /* Could be any value */);
		config_mocks(amount);

		assert_ok!(Loans::borrow(
			RuntimeOrigin::signed(BORROWER),
			POOL_A,
			loan_id,
			amount
		));
	});
}

#[test]
fn twice() {
	new_test_ext().execute_with(|| {
		let loan_id = util::create_loan(util::base_internal_loan());

		config_mocks(COLLATERAL_VALUE / 2);

		assert_ok!(Loans::borrow(
			RuntimeOrigin::signed(BORROWER),
			POOL_A,
			loan_id,
			COLLATERAL_VALUE / 2
		));
		assert_eq!(COLLATERAL_VALUE / 2, util::current_loan_debt(loan_id));

		assert_ok!(Loans::borrow(
			RuntimeOrigin::signed(BORROWER),
			POOL_A,
			loan_id,
			COLLATERAL_VALUE / 2
		));
		assert_eq!(COLLATERAL_VALUE, util::current_loan_debt(loan_id));

		// At this point the loan has been fully borrowed.
		let extra = 1;
		assert_noop!(
			Loans::borrow(RuntimeOrigin::signed(BORROWER), POOL_A, loan_id, extra),
			Error::<Runtime>::from(BorrowLoanError::MaxAmountExceeded)
		);
	});
}

#[test]
fn twice_with_elapsed_time() {
	new_test_ext().execute_with(|| {
		let loan_id = util::create_loan(util::base_internal_loan());

		config_mocks(COLLATERAL_VALUE / 2);

		assert_ok!(Loans::borrow(
			RuntimeOrigin::signed(BORROWER),
			POOL_A,
			loan_id,
			COLLATERAL_VALUE / 2
		));
		assert_eq!(COLLATERAL_VALUE / 2, util::current_loan_debt(loan_id));

		advance_time(YEAR / 2);

		assert_eq!(
			util::current_debt_for(
				util::interest_for(DEFAULT_INTEREST_RATE, YEAR / 2),
				COLLATERAL_VALUE / 2,
			),
			util::current_loan_debt(loan_id)
		);

		assert_ok!(Loans::borrow(
			RuntimeOrigin::signed(BORROWER),
			POOL_A,
			loan_id,
			COLLATERAL_VALUE / 2
		));

		// At this point the loan has been fully borrowed.
		let extra = 1;
		assert_noop!(
			Loans::borrow(RuntimeOrigin::signed(BORROWER), POOL_A, loan_id, extra),
			Error::<Runtime>::from(BorrowLoanError::MaxAmountExceeded)
		);
	});
}