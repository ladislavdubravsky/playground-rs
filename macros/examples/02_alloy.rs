use alloy::sol;

sol! {
    contract ContractA {
        function number() external view returns (uint256) {
            return 42;
        }
    }
}

/// cargo expand --example 02_alloy
fn main() {}
