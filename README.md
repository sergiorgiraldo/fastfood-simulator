# fastfood-simulator

Fast Food simulates the operation of a simple take-away restaurant and is
designed to help the developer put his or her knowledge of Promises and SOLID
design principles to work.

This app simulates customers of a take-away restaurant placing orders and
and waiting for them to be prepared and delivered to a pickup counter. After
placing the order the customer waits on the order to be announced before
picking it up and proceeding to the dining area.

The user stories that make up this app center around four distinct roles:

- User - the end user using the application
- Customer - the simulated Customer
- Order Taker - the simulated Order Taker
- Cook - the simulated Cook
- Server - the simulated Server

## Stack

- NextJS - Frontend
- Rust - API's

## Constraints

- Order tickets can be represented as two different types of Promises - one
the Server waits on while the Cook prepares the order and another the Customer
waits on while in the serving line.
- New customers arrive in the order line at a fixed interval of time. In other
words, new customers arrive at a constant rate.
- Order tickets are fulfilled at a fixed interval of time as well. They are
completed at a constant rate.

## User Stories

### Application Operation
-   [ ] User can see an input area that allows the entry of the time interval
for customer arrival and a time interval for the fulfilment of an 
_order ticket_ by the cook.
-   [ ] User can see a customized warning message if the customer arrival
interval or the order fulfilment interval is incorrectly entered.
-   [ ] User can start the simulation by clicking on a Start button.  
-   [ ] User can see an order line area containing a text box showing the 
number of customers waiting to place orders.
-   [ ] User can see an order area containing text boxes showing the
_order number_ currently being taken. 
-   [ ] User can see a kitchen area containing a text box showing the 
_order number_ that's being prepared and a text box listing the waiting 
orders in sequence, along with a count of the number of waiting orders.
-   [ ] User can see a Pickup area containing a text box showing the 
_order number_ that's currently available for pickup by the Customer and a
text box showing the number of Customers waiting in the serving line.
-   [ ] User can stop the simulation at any time by clicking a Stop button. 

## Idea

- [Fast Food Simulator - Logical Workflow](https://drive.google.com/file/d/1Thfm5cFDm1OjTg_0LsIt2j1uPL5fv-Dh/view?usp=sharing)
