// Group data with Records
type SuccessfulWithdrawal =
    { Amount: decimal
      Balance: decimal }

type FailedWithdrawal =
    { Amount: decimal
      Balance: decimal
      IsOverdraft: bool }

// Use discriminated unions to represent data of 1 or more forms
type WithdrawalResult =
    | Success of SuccessfulWithdrawal
    | InsufficientFunds of FailedWithdrawal
    | CardExpired of System.DateTime
    | UndisclosedFailure
// Returns a WithdrawalResult
let withdrawMoney amount = 
    printf "Withdrawal logic here %s" amount
    Success { Amount = 10.0M; Balance = 100.0M }

let handleWithdrawal amount =
    let w = withdrawMoney amount
    // The F# compiler enforces accounting for each case!
    match w with
    | Success s -> printfn $"Successfully withdrew %f{s.Amount}"
    | InsufficientFunds f -> printfn $"Failed: balance is %f{f.Balance}"
    | CardExpired d -> printfn $"Failed: card expired on {d}"
    | UndisclosedFailure -> printfn "Failed: unknown :("

handleWithdrawal "10"