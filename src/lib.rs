mod garden;

use crate::garden::Garden;
use crate::garden::vegetables::Vegetables;

#[cfg(test)]
mod tests {
    use crate::garden;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn list_vegetables() {
        crate::garden::vegetables::list_vegetables();

        garden::vegetables::list_vegetables();
    }
}

fn main() {}