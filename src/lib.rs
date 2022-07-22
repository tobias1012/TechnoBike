pub mod receiver;
pub mod bike;

use receiver::Receiver;

pub fn start_listener() -> receiver::Receiver {

    let mut recv = Receiver::new("1508");

    recv
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
