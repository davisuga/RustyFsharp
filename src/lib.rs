#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_attributes)]
pub mod RustyFsharp {
    use super::*;
    use fable_library_rust::DateTime_::DateTime;
    use fable_library_rust::Decimal_::decimal;
    use fable_library_rust::Decimal_::fromParts;
    use fable_library_rust::Microsoft::FSharp::Core::PrintfFormat_4;
    use fable_library_rust::Native_::on_startup;
    use fable_library_rust::Native_::Lrc;
    use fable_library_rust::String_::printf;
    use fable_library_rust::String_::string;
    use fable_library_rust::String_::stringFrom;
    use fable_library_rust::String_::toConsole;
    use fable_library_rust::System::IO::TextWriter;
    #[derive(Clone, Debug, Default, PartialEq, PartialOrd, Hash, Eq, Ord)]
    pub struct SuccessfulWithdrawal {
        pub Amount: decimal,
        pub Balance: decimal,
    }
    #[derive(Clone, Debug, Default, PartialEq, PartialOrd, Hash, Eq, Ord)]
    pub struct FailedWithdrawal {
        pub Amount: decimal,
        pub Balance: decimal,
        pub IsOverdraft: bool,
    }
    #[derive(Clone, Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
    pub enum WithdrawalResult {
        Success(Lrc<RustyFsharp::SuccessfulWithdrawal>),
        InsufficientFunds(Lrc<RustyFsharp::FailedWithdrawal>),
        CardExpired(DateTime),
        UndisclosedFailure,
    }
    pub fn withdrawMoney(amount: string) -> Lrc<RustyFsharp::WithdrawalResult> {
        (toConsole(printf(string("Withdrawal logic here %s")).clone()))(amount);
        Lrc::new(RustyFsharp::WithdrawalResult::Success(Lrc::new(
            RustyFsharp::SuccessfulWithdrawal {
                Amount: fromParts(100i32, 0i32, 0i32, false, 1u8),
                Balance: fromParts(1000i32, 0i32, 0i32, false, 1u8),
            },
        )))
    }
    pub fn handleWithdrawal(amount: string) {
        let w: Lrc<RustyFsharp::WithdrawalResult> = RustyFsharp::withdrawMoney(amount);
        match w.as_ref() {
            RustyFsharp::WithdrawalResult::InsufficientFunds(w_1_0) => {
                toConsole(stringFrom(format!("Failed: balance is {0}", &(w_1_0).Balance)).clone())
            }
            RustyFsharp::WithdrawalResult::CardExpired(w_2_0) => {
                toConsole(stringFrom(format!("Failed: card expired on {0}", w_2_0)).clone())
            }
            RustyFsharp::WithdrawalResult::UndisclosedFailure => {
                toConsole(printf(string("Failed: unknown :(")).clone())
            }
            RustyFsharp::WithdrawalResult::Success(w_0_0) => {
                toConsole(stringFrom(format!("Successfully withdrew {0}", &(w_0_0).Amount)).clone())
            }
        }
    }
    on_startup!(RustyFsharp::handleWithdrawal(string("10")));
}
