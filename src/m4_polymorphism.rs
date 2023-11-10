use ethers::types::Address;
use std::str::FromStr;

trait EthereumAddress {
    fn convert_address(&self) -> Result<Address, &'static str>;
}

// &str comes from rust
impl EthereumAddress for &str {
    fn convert_address(&self) -> Result<Address, &'static str> {
        match Address::from_str(self) {
            Ok(address) => Ok(address),
            Err(_) => Err("Invalid address"),
        }
    }
}

// Address comes from the library
impl EthereumAddress for Address {
    fn convert_address(&self) -> Result<Address, &'static str> {
        // dereference self, not get the pointer
        Ok(*self)
    }
}

// Generic type with Ethereum address as a trait
fn get_ethereum_data<T: EthereumAddress>(address: T) -> Address {
    // lazy way of doing this, does the match for you.
    let converted_address = address.convert_address().unwrap();
    converted_address
}

mod tests {
    use super::*;

    #[test]
    fn test_poly() {
        let addr: Address =
            Address::from_str("0xAb5801a7D398351b8bE11C439e05C5B3259aeC9B").unwrap();
        dbg!(addr);

        let new_addr: Address = get_ethereum_data("0xAb5801a7D398351b8bE11C439e05C5B3259aeC9B");
        assert_eq!(
            new_addr,
            Address::from_str("0xAb5801a7D398351b8bE11C439e05C5B3259aeC9B").unwrap()
        );
    }
}
