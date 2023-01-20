# Coffee Shop Simulation
 Using Rust's Structs, Impls to reconstruct a relatively complicated scenario 

I have used Rust to simulate the operations of a food court, in order to determine which system of seat reservation, the "chope" system or the first-come-first-served system, is more efficient.

First, I created a struct for customers, which contains fields such as whether or not they are using the chope system, their group size, the amount of time they spend eating, and whether or not they have queued, been seated, and finished eating. I then used a Poisson distribution to randomly generate customers based on the time of day, with more customers arriving during peak hours.

The Customer struct is used to represent each customer that enters the food court. It has several fields, such as chope, group_size, current, eating_time, stalls, time_spent, queued, tabled, and eaten, that are used to keep track of the customer's status and actions. The chope field is a boolean that represents whether the customer has reserved a table before arriving at the food court. The group_size field represents the size of the customer's group, and is used to determine how many tables they need. The current field represents the amount of time the customer has left before they can take their next action. The eating_time field represents the amount of time the customer will spend eating. The stalls field is a vector that represents the food stalls that the customer will visit. The time_spent field is the amount of time the customer has spent in the food court so far. The queued field is a boolean that represents whether the customer is currently in a queue. The tabled field is a boolean that represents whether the customer has a table reserved. The ``eaten``field together with the current fields are the critical elements of the struct that indicate to the implemented methods that this customer has completed their visit.

```rs
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
```


Next, I implemented the concept of tables and waiting lists, using arrays and vectors in Rust. The tables were represented by an array, and the waiting lists were represented by vectors, which can change in length. The tables are occupied and update their status over time, while the waiting lists keep track of customers who are waiting for tables.

the loop implementation is the core of the simulation. It sets up the necessary variables and data structures, such as the seats array to keep track of table occupancy and the waiting_list vector to keep track of customers waiting for tables. It also initializes the stall_clog_times array to keep track of the order times for each stall, and the customer vector to keep track of all customers in the food court.

The loop then uses the Poisson distribution to generate customers based on the time of day, with more customers arriving during peak hours. Inside the loop, the update method for the Customer struct is called to guide the customer over time, taking into account the time spent in queues, at tables, and eating. The loop also implements functions to filter customers who have completed their visit to the food court, clear the chope system of customers who have exceeded the 10-minute waiting time, and generate new customers based on their behavior type.

In this way, the loop simulates the operations of a food court and allows me to determine which system of seat reservation, the chope system or the first-come-first-served system, is more efficient. The loop is designed to be flexible and can be adapted to other similar scenarios, making it useful in determining the most efficient system in any food court setting.

In this way, I was able to simulate the food court operations, and determine that the chope system is more efficient for serving food during peak hours, as it leads to fewer people waiting in queues. This solution can be easily adapted to other similar scenarios and will be useful in determining the most efficient system in any food court setting.


# Results

Here is an example for 5000 repeated simulations. Simulated a rate of 2.8 for the first period, followed by 1.4 for the 60 and 45 minute periods respectively. Results are comparing scenarios where ALL customers reserve places, vs ALL customers do not do so.

## Sales

![Sales](https://user-images.githubusercontent.com/100022747/213763864-cccff059-c500-4256-9122-254724e3ab41.png)

## Time Distribution

Distribution of time spent for customers. 

![TimeDistribution](https://user-images.githubusercontent.com/100022747/213764065-314877b6-ac9e-43d3-9052-84a2a0d760c7.png)

## Number of customers who entered court

![Patrons](https://user-images.githubusercontent.com/100022747/213764342-c41a09e4-c560-4cba-9957-c99bc9a88acc.png)

## Number of customers who remain in the food court 

This is after peak hour simulation is over


![Remaining_Customers](https://user-images.githubusercontent.com/100022747/213764517-d05c8539-1730-4f00-b6f7-6cc27559f4bc.png)


## Number of customers queueing for tables by end of simulation


![Waittable_atend](https://user-images.githubusercontent.com/100022747/213764588-33704d62-7bc7-4f2f-b23b-d2bbf2b9e25a.png)



