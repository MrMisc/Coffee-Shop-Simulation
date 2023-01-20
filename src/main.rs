
// use csv;// use rand::distributions::distribution::Distribution;
use rand::distributions::Uniform;
use rand::distributions::{Distribution, Standard};
use rand::{thread_rng, Rng};
use statrs::distribution::{Normal, Poisson, StudentsT, Triangular, Weibull};

//Construct a Customer type
#[derive(Clone)]
pub struct Customer {
    chope: bool,
    group_size: u8,
    current: u32,
    eating_time: u32,
    stalls:Vec<u8>,
    time_spent: u32,
    queued: bool,
    tabled: bool,
    eaten: bool,
}
//Update, THEN exit
impl Customer {
    //Customer vector filtered for those who have completed visit to court
    fn exit(vector: Vec<Customer>, time_spent: &mut Vec<u32>) -> Vec<Customer> {
        // let mut new_vector:Vec<Customer> =  Vec::new();
        for i in &vector {
            if (i.group_size == 0 && i.current==0) || (i.eaten && i.current==0) {
                if i.group_size != 0 {
                    for ele in 0..i.group_size as usize {
                        time_spent.push(i.time_spent);
                    } //if more than 1 customer
                } else {
                    time_spent.push(i.time_spent); // if takeaway
                }
            }
        }
        vector
            .into_iter()
            .filter(|i| ((i.group_size == 0 && i.current==0) || (i.eaten && i.current==0)) == false)
            .collect()
    }
    fn clear_chope(vector:Vec<Customer>)->Vec<Customer>{ //have already established previously that these customers will end up leaving due to time violation
        //This command will update and clear the entries that have exceeded 10min
        vector.into_iter().filter_map(|mut x| {
            x.time_spent += 1;
            if x.time_spent < 10 {
                Some(x)
            } else {
                None
            }
        }).collect()
    }
    fn get_time(&self) -> u32 {
        self.current
    }
    fn update(&mut self,clogs: &mut [u32; 5],ordering_times: &[f64; 5], orders: &mut u32) {
        //put at end of loop to check if customer is available
        if self.current > 0 {
            self.current -= 1;
        }
        self.time_spent += 1;
        //Chope routine
        if self.chope && self.current == 0 {
            if self.tabled == false {
                if self.group_size>0{
                    self.tabled = true;
                }else if self.group_size==0{*orders+=self.order() as u32;}
            } else if self.queued == false && self.group_size>0{
                &mut self.queue(clogs, &ordering_times);                
                self.queued = true;                                
            } else if self.eaten == false && self.group_size>0{
                *orders += self.order() as u32;
                self.current = self.eating_time;
                self.eaten == true;
            }//NON chope routine
        } else if self.chope == false && self.current == 0{
            if self.queued == false {
                self.queued = true;         
                *orders +=self.order() as u32;       
            }else if self.tabled && self.eaten == false && self.group_size>0{
                self.current = self.eating_time;
                self.eaten = true;
            }
        }
    }
    //Generate new customer type - needs separately generated group type though [CHOPING]
    //   Also need to check for yourself ideally, how long this choping customer has to wait to do their first action
    fn gen_chope(clog_times: &[u32; 5], ordering_times: &[f64; 5]) -> Customer {
        let group: u8 = choose_customer([29, 21, 45, 15, 8, 1, 1, 1, 2]);
        let mut stalls: Vec<u8> = Vec::new();
        for i in 0..group as usize {
            stalls.push(choose_stall());
        }        
        let mut queue_time: u32 = 0;
        for i in &stalls {
            // clog_times[*i as usize] += ordering_times[*i as usize] as u32;
            let mut ind_time: u32 = clog_times[*i as usize] + ordering_times[*i as usize] as u32;
            if queue_time < ind_time {
                queue_time = ind_time;
            }
        }        
        let stall_eating_times: [[f64; 2]; 5] = [
            [23.0, 3.0],
            [16.0, 2.0],
            [18.0, 2.0],
            [20.0, 2.5],
            [17.0, 1.5],
        ]; //remains same regardless of day!
        let mut eats: u32 = 0;
        for i in &stalls {
            let time_for_meal: f64 = normal(
                stall_eating_times[*i as usize][0],
                stall_eating_times[*i as usize][1],
            );
            if eats < time_for_meal as u32 {
                eats = time_for_meal as u32;
            }
        }        
        Customer {
            chope: true,
            group_size: group,
            current: 0,
            eating_time: eats, //perpetually zero here for chopers. Time decided and logged externally
            stalls,
            time_spent: 0,
            queued: false,
            tabled: false,
            eaten: false,
        }
    }
    //Generate new customer [NO CHOPING]
    //Need to decide queue times from choices to put into current at start
    fn gen(clog_times: &[u32; 5], ordering_times: &[f64; 5]) -> Customer {
        let group: u8 = choose_customer([29, 21, 45, 15, 8, 1, 1, 1, 2]);
        //Those that do not chope, decided beforehand what they want to eat and hence, we should randomise it here for ourselves
        let mut stalls: Vec<u8> = Vec::new();
        for i in 0..group as usize {
            stalls.push(choose_stall());
        }
        let mut queue_time: u32 = 0;
        for i in &stalls {
            // clog_times[*i as usize] += ordering_times[*i as usize] as u32;
            let mut ind_time: u32 = clog_times[*i as usize]+ ordering_times[*i as usize] as u32;
            if queue_time < ind_time {
                queue_time = ind_time;
            }
        }

        let stall_eating_times: [[f64; 2]; 5] = [
            [23.0, 3.0],
            [16.0, 2.0],
            [18.0, 2.0],
            [20.0, 2.5],
            [17.0, 1.5],
        ]; //remains same regardless of day!
        let mut eats: u32 = 0;
        for i in &stalls {
            let time_for_meal: f64 = normal(
                stall_eating_times[*i as usize][0],
                stall_eating_times[*i as usize][1],
            );
            if eats < time_for_meal as u32 {
                eats = time_for_meal as u32;
            }
        }
        //Output
        Customer {
            chope: false,
            group_size: group,
            current: queue_time,
            eating_time: eats,
            stalls,
            time_spent: 0,
            queued: false,
            tabled: false,
            eaten: false,
        }
    }
    //actually queue up when appropriate
    fn queue(&mut self, clogs: &mut [u32; 5], ordering_times: &[f64;5]){
        let mut queue_time:u32 = 0;
        for i in &self.stalls{
            if clogs[*i as usize]>queue_time{
                queue_time = clogs[*i as usize].clone();
            }
        }
        self.current+=queue_time;
        for i in &self.stalls{
            clogs[*i as usize]+= ordering_times[*i as usize] as u32;
        }
    }
    //Function to check if customer is available to do next action
    fn available(&self) -> bool {
        self.current == 0
    }
    //Order food
    fn order(&self) -> u8 {
        let group_type: u8 = self.group_size;
        //Takeaway group
        if group_type == 0 {
            let mut rng = thread_rng();
            let roll = Uniform::new(0.0, 1.0);
            let rollnumber: f64 = rng.sample(roll);
            let order_amount: [f64; 3] = [0.7, 0.9, 1.0];
            let no: [u8; 3] = [1, 2, 4];
            let mut count: u8 = 0;
            for i in order_amount {
                if rollnumber < i {
                    break;
                }
                count += 1;
            }
            return no[count as usize];
        } else {
            return group_type;
        }
    }
}

//Function to randomly choose group number/customer type
// Function to embody choosing customer type - V
pub fn choose_customer(types: [u32; 9]) -> u8 {
    let mut rng = thread_rng();
    let roll = Uniform::new(0.0, 1.0);
    let sum_my_types: u32 = types.iter().sum();
    let rollnumber: f64 = rng.sample(roll) * (sum_my_types as f64);
    let mut count: u8 = 0;
    let mut countie: u32 = 0;
    for i in types {
        countie += i;
        if (rollnumber as u32) <= countie {
            break;
        }
        count += 1;
    }
    count
}

pub fn choose_stall() -> u8 {
    let mut rng = thread_rng();
    let roll = Uniform::new(0.0, 1.0);
    let rollnumber: f64 = rng.sample(roll);
    let stall_prob_list: [f64; 5] = [0.14, 0.31, 0.25, 0.18, 0.12];
    let mut count: u8 = 0;
    let mut prob_total: f64 = 0.0;
    for i in stall_prob_list {
        prob_total += i;
        if rollnumber < prob_total {
            break;
        }
        count += 1;
    }
    count
}

pub fn normal(mean: f64, std: f64) -> f64 {
    let mut thing: f64 = 0.0;
    loop {
        let mut rng = thread_rng();
        let v: &Vec<f64> = &Normal::new(mean, std)
            .unwrap()
            .sample_iter(&mut rng)
            .take(1)
            .collect();
        thing = v[0];
        if thing > 0.0 {
            break;
        }
    }
    thing
}

//Have customer of interest, log interest in a table! - this means that capacity (if in multiple) is not a potential limiting factor. It should already be accounted for in main code

//Let's get specific in case you get confused later!

//If you are choping, that means you will be logging a table, BEFORE you go to queue, and this updates the time inside the choped table array BEFORE going to queue so you have to decide immediately within this function
//what shops they are queueing at, AND take the MAX of the queue + cooking times for each customer for each unique stall, PLUS the maximum of eating time amongst the customers for each of their meals

//If NOT choping, you are logging interest in a table after already having gotten your food, and it is just a matter of potentially waiting on the waiting list vector if all tables are full indefinitely! But queue time is not counted inside here!
//The waiting time from waiting for a table is added with the concurrently calculated eating time (of which you take the max of if in a group - but you would be eating at the same time together so you would )

//This depends on
// availability (must be available for all customers in group if 2 or more)
// and customer group type (to log the time)

//Generalizing to customer behaviour type here
pub fn table_log(
    cust: &mut Customer,
    seating: &mut [u32; 48],
    wait_list: &mut Vec<Waiting_List>,
    eat: &[[f64; 2]; 5],
    gather: &[f64; 5],
    clogs: &mut [u32; 5],
){
    //output is to be appended to table array
    let customer_type: u8 = cust.group_size;
    let chope: bool = cust.chope;
    // let mut output_order:u32 = 0;
    if chope { // NOTE this part of the command actually is not necessary anymore (where it chooses stalls and queue time for the customers). This is dynamically done automatically by Customer, but might as well leave it here as a learning
        //lesson. Essentially, when we generate a Customer, it will have an in built choice of stalls. with queue times ready to be read WHEN THEY ARE starting to queue (if current ==0 AND queue= false and tabled== true)
        //This set of code overwrites and redoes those choices. To make sure we are not double dipping in queues, we make sure to set queued to true as well.

        //This means that you have to check for the 10min rule for chopers BEFORE applying this function
        cust.tabled = true;
        //This is pre-supposing that there is already sufficient space for a table to hold the group to be choped
        let mut stall_choices: Vec<u8> = Vec::new();
        for i in 0..customer_type as usize {
            //what stall did each customer choose?
            stall_choices.push(choose_stall());
        }
        //Before modifying this list to remove duplicate queue times, let's simulate all the normal values and find max of eating times
        let mut eating_time: f64 = 0.0;
        for i in &stall_choices {
            let latest: f64 = normal(eat[*i as usize][0], eat[*i as usize][1]);
            if eating_time < latest {
                eating_time = latest;
            }
        }
        cust.eating_time = eating_time as u32; //log for the hell of it. Not meaningful for chope model

        //Take maximum of gathering food time
        //CHANGE THE BELOW TO ACCOUNT FOR MULTIPLE COOKING TIMES NEEDED FOR MULTIPLE ORDERS PER PERSON IN QUEUE
        let mut gather_time: f64 = 0.0;
        for stall in &stall_choices {
            if gather_time < (clogs[*stall as usize] as f64) + gather[*stall as usize] {
                gather_time = (clogs[*stall as usize] as f64) + gather[*stall as usize];
            }
        }

        cust.current = gather_time as u32; // chopers go queue and have to wait maximum time for queue
        cust.queued = true; //allows us to bypass the automatic queueing system that happens for Customer 
        //Update clog times AFTER checking for maximum wait times to maintain independence
        for stall in stall_choices {
            clogs[stall as usize] += gather[stall as usize] as u32; //update clog time to stall for the queue
        }

        //TABLES OCCUPIED BELOW
        //update table logs - occupation
        //use this value to occupy the zeroes available
        let time: u32 = (gather_time + eating_time) as u32;
        seating.sort();
        for i in 0..customer_type as usize {
            seating[i] = time;
        }
        // time
    } else {
        //You have presumably queued already, so table logging here is going to include WAITING TIME for a table (indefinitely) and then max eating time
        //Since not choped beforehand, you do NOT assume that tables are available beforehand for the non chopers.
        //Check tables and then waiting list
        // cust.queued = true;
        // output_order = cust.order() as u32;
        //IF AVAILABLE TABLES
        if check_tables(customer_type, &seating) {
            seating.sort();
            for i in 0..customer_type as usize {
                // println!("Customer's eating time {} is being logged", cust.eating_time);
                seating[i] = cust.eating_time; //<- time decided BEFORE coming to table
            }
            cust.current = cust.eating_time;
            //Customer(s) now seated
            //Log it
            cust.tabled = true;
        } else {
            //NO AVAILABLE TABLES
            //Wait indefinitely | ie 100% being added to waiting list
            cust.current += Waiting_List::add(customer_type, cust.eating_time, wait_list, &seating);
            // cust.current += cust.eating_time; //account for eating time and add to customer
            cust.tabled = true;
        }
    }
    // output_order
}

pub fn check_tables(group_size: u8, listoftables: &[u32; 48]) -> bool {
    let mut count: u8 = 0;
    for i in listoftables.clone() {
        if i == 0 {
            count += 1;
        }
    }
    group_size <= count
}

//Call this function if tables are full and you are going to wait.

//It should only check time, not update waiting list. Must do that separately
pub fn to_wait(group_size: u8, eating_time: u32, seating: &[u32; 48]) -> (u32, [u32; 48]) {
    let mut checkie = seating.clone().to_vec();
    checkie.sort();
    checkie.dedup();

    let mut newvector: [u32; 48] = [1; 48];
    //duplicate vec again
    let mut count: u8 = 0;
    for i in seating {
        newvector[count as usize] *= i;
        count += 1;
    }
    let mut counter: u32 = 0;
    for it in checkie {
        let mut count: u8 = 0;
        for i in seating {
            // println!("{} minutes until this spot is free",i);
            if deduct(*i, it).1 {
                count += 1
            };
        }
        if count >= group_size {
            // println!("Will have space after {} minutes!", it);
            for i in 0..newvector.len() {
                newvector[i] = deduct(newvector[i], it).0;
            }
            newvector.sort();
            for ele in 0..group_size as usize {
                newvector[ele] = eating_time;
            }
            counter = it as u32;
            break;
        }
    }

    (counter, newvector)
}

fn deduct(no: u32, minus: u32) -> (u32, bool) {
    if no <= minus {
        return (0, true);
    } else {
        return (no - minus, false);
    }
}

//Vector Table struct
// record 4 things: group size, eating time, waiting time, resultant list
#[derive(Clone,Debug)]
pub struct Waiting_List {
    group_size: u8,
    eating_time: u32,
    waiting_time: u32,
    future_list: [u32; 48],
}

impl Waiting_List {
    //Add entry by stating customer group size and eating time and vector of waiting list and main table list in case
    //      Waiting_List::add(group_size, eating_time, vector_of_waitlist, seats);
    fn add(
        group_size: u8,
        eating_time: u32,
        vector_of_waitlist: &mut Vec<Waiting_List>,
        ifelse_maintables: &[u32; 48],
    ) -> u32 {
        let mut vector: &[u32; 48] = &[0; 48];
        if vector_of_waitlist.len() == 0 {
            vector = ifelse_maintables;
        } else {
            vector = &vector_of_waitlist[vector_of_waitlist.len() - 1].future_list;
        }
        let waiting_for_config: (u32, [u32; 48]) = to_wait(group_size, eating_time, vector);
        let mut cumulative_wait_time: u32 = waiting_for_config.0;
        if vector_of_waitlist.len() >= 1 {
            cumulative_wait_time += vector_of_waitlist[vector_of_waitlist.len() - 1].waiting_time;
        }
        vector_of_waitlist.push(Waiting_List {
            group_size,
            eating_time,
            waiting_time: cumulative_wait_time,
            future_list: waiting_for_config.1,
        });
        cumulative_wait_time
    }

    fn exit(&mut self, seats: &mut [u32; 48]) {
        //outputs customers to the seats
        if self.waiting_time == 0 {
            seats.sort();
            for i in 0..self.group_size as usize {
                seats[i] = self.eating_time;
            }

            //remove entry
            // if vector_of_waitlist.len()==1{
            //     *vector_of_waitlist = Vec::new();
            // }
            // else if vector_of_waitlist.len()>1{
            //     vector_of_waitlist.remove(0);
            // }
        }
    }
    //So without having to call exit, you should be able to just call Waiting_List::update(&mut seats, &mut waiting_list)
    fn update(
        seats: &mut [u32; 48],
        mut vector_of_waitlist: Vec<Waiting_List>,
    ) -> Vec<Waiting_List> {
        // for i in vector_of_waitlist{
        //     if i.waiting_time>0{i.waiting_time-=1;}
        //     if i.waiting_time==0{
        //         i.exit(seats,&mut vector_of_waitlist);
        //     }
        // }
        if vector_of_waitlist.len() == 1 {
            vector_of_waitlist[0].waiting_time = deduct(vector_of_waitlist[0].waiting_time, 1).0;
            if vector_of_waitlist[0].waiting_time == 0 {
                vector_of_waitlist[0].exit(seats);
                return Vec::new();
            } else {
                return vector_of_waitlist;
            }
        } else {
            let vectors_to_exit: Vec<Waiting_List> = vector_of_waitlist
                .clone()
                .into_iter()
                .filter(|x| x.waiting_time == 1)
                .collect();
            for mut i in vectors_to_exit {
                i.exit(seats);
            }
            let mut output_vector: Vec<Waiting_List> = vector_of_waitlist
                .clone()
                .into_iter()
                .filter(|x| x.waiting_time >= 1)
                .collect();
            for mut i in &mut output_vector {
                i.waiting_time -= 1;
            }
            return output_vector;
        }
        // vector_of_waitlist.into_iter().filter(|x| )
    }
}




fn main(){
    for _i in 0..5000000{
        toloop();
    }
}












fn toloop() {
    //User variables
    let rate_base: f64 = 2.8;

    //main based variables

    const CAPACITY: u8 = 60;
    let mut seats: [u32; 48] = [0; 48];
    let mut waiting_list: Vec<Waiting_List> = Vec::new();

    //Stall Time Dist Info
    let stall_order_times: [f64; 5] = [2.0, 1.0, 1.5, 1.5, 2.0];
    let stall_eating_times: [[f64; 2]; 5] = [
        [23.0, 3.0],
        [16.0, 2.0],
        [18.0, 2.0],
        [20.0, 2.5],
        [17.0, 1.5],
    ];

    //Stall time log
    let mut stall_clog_times: [u32; 5] = [0, 0, 0, 0, 0];
    let mut stall_sales: [u32; 5] = [0, 0, 0, 0, 0];

    //Customer Vector log
    //Log customer types AND times that customer have to wait
    //each entry should be reduced by 1 per loop iteration
    //entry for customer must be 0 for any next action to be taken
    let mut customer: Vec<Customer> = Vec::new();

    let mut choping:Vec<Customer> = Vec::new();

    //Customer Type Vector | To be edited by User
    let customer_type: [u32; 9] = [29, 21, 45, 15, 8, 1, 1, 1, 2];

    let mut time_spent: Vec<u32> = Vec::new();
    let mut orders: u32 = 0;

    let mut total_visitors:u32 = 0; //People who consider the court initially at least
    let mut total_patrons:u32 = 0; //People who enter the shop with a willingness to buy

    let mut time: u64 = 0;
    //Chope model first
    loop {
        //Generate Customers off of Poisson
        // let mut rng = thread_rng();
        // let mut number:u32 = Poisson::new(rate).unwrap() as u32;
        let mut rng = thread_rng();
        let mut rate: f64 = 0.0;
        if time>60{
            rate=rate_base/3.*2.0;
        }else{
            rate=rate_base;
        }
        let v: &Vec<f64> = &Poisson::new(rate)
            .unwrap()
            .sample_iter(&mut rng)
            .take(1)
            .collect();
        // println!("Let's try to print a Poisson number {}", v[0]);
        let number: u32 = v[0] as u32;
        for i in 0..number as usize {
            let mut cust: Customer = Customer::gen_chope(&stall_clog_times, &stall_order_times);
            //For each, check if there is capacity for all at same time EXCEPT for takeaway (group size 0)
            if cust.group_size == 0 {
                total_patrons+=1;
                total_visitors+=1;
                //choose stall
                let stall_choice: u8 = choose_stall();
                //add to clog time and add as queue number to current for customer before pushing
                stall_clog_times[stall_choice as usize] +=
                    stall_order_times[stall_choice as usize] as u32;
                let mut mini_time: u32 = stall_clog_times[stall_choice as usize];
                cust.current = mini_time;
                customer.push(cust);
            } else {
                //Not takeout group
                total_visitors+=cust.group_size as u32;
                //If tables available...
                if check_tables(cust.group_size, &seats) && waiting_list.len()==0 {
                    table_log(
                        &mut cust,
                        &mut seats,
                        &mut waiting_list,
                        &stall_eating_times,
                        &stall_order_times,
                        &mut stall_clog_times,
                    );
                    total_patrons+=cust.group_size as u32;
                    customer.push(cust);                    
                }else {
                    // println!("Tables full, waiting time to be {}, to change the current seating from {:?} to {:?}", to_wait(cust.group_size, cust.eating_time, &seats).0, &seats, to_wait(cust.group_size, cust.eating_time, &seats).1);
                    if to_wait(cust.group_size, cust.eating_time, &seats).0<=10 && waiting_list.len()==0 || waiting_list.len()>0 && to_wait(cust.group_size, cust.eating_time, &waiting_list[waiting_list.len()-1].future_list).0+waiting_list[waiting_list.len()-1].waiting_time<=10{
                        cust.current = Waiting_List::add(cust.group_size, cust.eating_time, &mut waiting_list, &seats);
                        cust.tabled=true;
                        total_patrons+=cust.group_size as u32;
                        customer.push(cust);                    
                }else{choping.push(cust);}
                //Remaining customers that see that it takes too much time are appended to choping vector that is regularly updated with time
                
            }
            }
        }
        // println!("Seating is as follows");
        // dbg!(seats.clone());
        //Updating time spent for each customer AND exit for those who have completed eating (or ordering for takeout)
        for mut i in &mut customer { //customeris our vector of all customers
            i.update(&mut stall_clog_times, &stall_order_times, &mut orders);  //update is to 
        }
        customer = Customer::exit(customer, &mut time_spent);
        choping = Customer::clear_chope(choping);
        //Update waiting list
        waiting_list = Waiting_List::update(&mut seats, waiting_list);

        // dbg!(waiting_list.clone());
        //Update stall clogs
        for mut i in stall_clog_times {
            i -= deduct(i, 1).0;
        }
        //Update seats
        for mut i in seats {
            i -= deduct(i, 1).0;
        }
        //Time breaking conditions
        time += 1;
        if time >= 105 {
            break;
        }
    }
    // dbg!(time_spent);
    let in_wait:u32 = (waiting_list.clone().len() + choping.clone().len()) as u32;
    let cust_lag:u32 = customer.clone().len() as u32;
    // let no_of_orders:u32 = orders.clone();

    //Readable print results ----------------------------------------------------------

    // println!("Total number of {} orders have been processed", orders );

    // println!("A total of {} people passed by and/or entered the court", total_visitors);
    // println!("A total of {} people, have entered to purchase food from the court", total_patrons);

    // println!("{} customers are still waiting for a table, {} of them will be leaving, having not ordered anything", waiting_list.len() + choping.len(), choping.len());
    // println!("{} total customers are still inside the food court", customer.len());
    // println!("NON CHOPE RESULTS FOLLOWING");


    //----------------------------------------------------------------------------------------------------
    //NON CHOPE SIMULATION
    //redefine mutable vars
    let mut seats: [u32; 48] = [0; 48];
    let mut waiting_list: Vec<Waiting_List> = Vec::new();    
    let mut stall_clog_times: [u32; 5] = [0, 0, 0, 0, 0];
    let mut stall_sales: [u32; 5] = [0, 0, 0, 0, 0];

    //Customer Vector log
    //Log customer types AND times that customer have to wait
    //each entry should be reduced by 1 per loop iteration
    //entry for customer must be 0 for any next action to be taken
    let mut customer: Vec<Customer> = Vec::new();

    //Customer Type Vector | To be edited by User
    // let customer_type: [u32; 9] = [29, 21, 45, 15, 8, 1, 1, 1, 2];

    let mut time_spent2: Vec<u32> = Vec::new();
    let mut orders2: u32 = 0;

    let mut total_visitors2:u32 = 0; //People who consider the court initially at least
    let mut total_patrons2:u32 = 0; //People who enter the shop with a willingness to buy

    let mut time: u64 = 0;    
    loop{
        let mut rng = thread_rng();
        let mut rate: f64 = 0.0;
        if time>60{
            rate=rate_base/3.*2.0;
        }else{
            rate=rate_base;
        }
        let v: &Vec<f64> = &Poisson::new(rate)
            .unwrap()
            .sample_iter(&mut rng)
            .take(1)
            .collect();
        // println!("Let's try to print a Poisson number {}", v[0]);
        let number: u32 = v[0] as u32;        
        for i in 0..number as usize {
            let mut cust: Customer = Customer::gen(&stall_clog_times, &stall_order_times);
            //For each, check if there is capacity for all at same time EXCEPT for takeaway (group size 0)
            if cust.group_size <= 1 {
                total_patrons2+=1;
                total_visitors2+=1;
                //choose stall
                let stall_choice: u8 = choose_stall();
                //add to clog time and add as queue number to current for customer before pushing
                stall_clog_times[stall_choice as usize] +=
                    stall_order_times[stall_choice as usize] as u32;
                let mut mini_time: u32 = stall_clog_times[stall_choice as usize];
                cust.current = mini_time;
                customer.push(cust);        
            }else if check_tables(cust.group_size, &seats) && waiting_list.len()==0{
                //There are still seats left
                cust.queue(&mut stall_clog_times, &stall_order_times);
                total_patrons2+=cust.group_size as u32;
                total_visitors2+=cust.group_size as u32;                
                customer.push(cust);
            }else{
                //Calculate HOW MANY PEOPLE are waiting for tables
                let mut calc:u32 = 0;
                for i in waiting_list.clone(){
                    calc+=i.group_size as u32;
                }
                if calc<12{
                    cust.queue(&mut stall_clog_times, &stall_order_times);
                    total_patrons2+=cust.group_size as u32;
                    total_visitors2+=cust.group_size as u32;                
                    customer.push(cust);                    
                }else{
                    total_visitors2+=cust.group_size as u32;
                }
            }

    }
        //---------------------------------------------------------
        //Non chopers specifically have to be put to set to find a table with table_log and not "Naturally" switch to tabled=true automatically. 
        // println!("Seating is as follows");
        // dbg!(seats.clone());
        //Did not set this mechanic within Customer because it also requires the waiting list vector as an argument, and we decided to just manually check it here
        for mut i in &mut customer{
            // if i.current==0 && i.group_size>0{println!("Queue status:{} current:{} group_size:{}", i.queued, i.current, i.group_size);}
            if i.queued==true && i.current==0 && i.group_size>0{
                // println!("Logging tables");
                table_log(&mut i, &mut seats, &mut waiting_list, &stall_eating_times, &stall_order_times, &mut stall_clog_times);
            }
        }
        //Updating time spent for each customer AND exit for those who have completed eating (or ordering for takeout)
        for mut i in &mut customer { //customeris our vector of all customers
            i.update(&mut stall_clog_times, &stall_order_times, &mut orders2);  //update is to 
        }
        customer = Customer::exit(customer, &mut time_spent2);
        //Update waiting list
        waiting_list = Waiting_List::update(&mut seats, waiting_list);

        // dbg!(waiting_list.clone());
        //Update stall clogs
        for mut i in stall_clog_times {
            i -= deduct(i, 1).0;
        }
        //Update seats
        for mut i in seats {
            i -= deduct(i, 1).0;
        }
        //Time breaking conditions
        time += 1;
        if time >= 105 {
            break;
        }            
}
    // dbg!(time_spent);
    // println!("Total number of {} orders have been processed", orders2 );

    // println!("A total of {} people passed by and/or entered the court", total_visitors2);
    // println!("A total of {} people, have entered to purchase food from the court", total_patrons2);

    // println!("{} customers are still waiting for a table", waiting_list.len());
    // println!("{} total customers are still inside the food court", customer.len());

    let in_wait2:u32 = (waiting_list.clone().len() + choping.clone().len()) as u32;
    let cust_lag2:u32 = customer.clone().len() as u32;

    //Outputting results as print-------------------------------------------------

    println!("{} {} {} {} {} {} {} {} {} {}", orders, total_visitors, total_patrons, in_wait, cust_lag, orders2, total_visitors2, total_patrons2, in_wait2, cust_lag2);
    println!("{:?}",time_spent);
    println!("{:?}",time_spent2);

}
