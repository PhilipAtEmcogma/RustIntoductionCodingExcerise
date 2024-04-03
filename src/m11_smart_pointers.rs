
#[cfg(test)]
mod tests{
use std::rc::{Rc, Weak};
use std::cell::RefCell;

    #[test]
    fn tests_box_smart_pointer(){

        let x = Box::new(50); //force saving variable in heap instead of stack

    }


    #[test]
    fn tests_reference_counter(){
        // let x = Rc::new(RefCell::new(50)); //x is now a refference counter, if we use RefCell to borrow and change number, x doesn't need to be mut anymore
        
        // //refference counter is a smart counter that keep tract of number of refference to a value
        // let y = Rc::clone(&x); //creates a new pointer to an existing data, and increment Rc counter by 1, such that it doesn't copy all the data in a variable to safe space (incase the variable is struct or large dataset)


        // *x.borrow_mut() = 70; //derefferenced and borrowed, in combination to RefCell::new(50), means the new vaule to x here will be the new value to x above as well
        // dbg!(y);
        // dbg!(x);
        // //refference counting

        #[derive(Debug, Clone)]
        struct House{
            address_number: u16,
            street: String,
            furniture: RefCell<Vec<Rc<Furniture>>>
        }

        #[derive(Debug, Clone)]
        struct Furniture{
            id: String,
            description: String,
            house: Weak<House> //weak refference.
        }

        let mut house_1 = Rc::new(House{
            address_number: 123,
            street: "coding ave".to_string(),
            furniture: RefCell::new(vec!())

        });

        let table = Rc::new(Furniture{
            id: "table1".to_string(),
            description: "kitchen table".to_string(),
            house: Rc::downgrade(&house_1)
        });

        let desk = Rc::new(Furniture{
            id: "desk1".to_string(),
            description: "office desk".to_string(),
            house: Rc::downgrade(&house_1)
        });
        
        house_1.furniture.borrow_mut().push(Rc::clone(&table));
        house_1.furniture.borrow_mut().push(Rc::clone(&desk));

        dbg!(house_1);

    }

}