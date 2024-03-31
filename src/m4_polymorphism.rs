use ethers::types::Address;
use std::str::FromStr;

trait EthereumAddress{
    //if returning a string, the string needs to be a lifetime static, hence &'static str
    fn convert_address(&self) -> Result<Address, &'static str>;
}

impl EthereumAddress for &str{
    fn convert_address(&self) -> Result<Address, &'static str>{
        //check can string address be converted to a H160 address, if yes, do so, if not, print warning
        match Address::from_str(self){
            Ok(address) => Ok(address),
            Err(_) => Err("Invalid Ethereum Address String")
        }
    }
}

impl EthereumAddress for Address{
    fn convert_address(&self) -> Result<Address, &'static str>{
        Ok(*self)
    }
}

//function that accepts a string or string reference
fn get_etherum_data<T: EthereumAddress>(address: T)->Address{
    let convert_address = address.convert_address().unwrap();
    convert_address

}

//unit test
#[cfg(test)]
mod test{
    //use super to import everything
    use super::*;

    #[test]
    fn tests_poly(){
        //passing address in as string
        let addr:Address = Address::from_str("0x388C818CA8B9251b393131C08a736A67ccB19297").unwrap();
        let new_addr: Address = get_etherum_data(addr);
        assert_eq!(new_addr, Address::from_str("0x388C818CA8B9251b393131C08a736A67ccB19297").unwrap());

        //passing address in as is
        let new_addr: Address = get_etherum_data("0x388C818CA8B9251b393131C08a736A67ccB19297");
        assert_eq!(new_addr, Address::from_str("0x388C818CA8B9251b393131C08a736A67ccB19297").unwrap());



    }
}