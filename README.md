# net-worth

This CLI app calculates salary payments, compound interest, and dollar-cost averaging.


# How to run

```cargo run {command} {args}```

The following commands are:
- salary
- compound
- average_share_price

You can type --help as an arg for more documentation. 


# How to use the average_share_price command for calculating dollar-cost averaging?

<img width="506" alt="Screen Shot 2021-06-24 at 7 00 13 PM" src="https://user-images.githubusercontent.com/43818256/123342824-7abeec80-d51e-11eb-83d0-13a5c1d3a51a.png">
In this example, we have two integers on the left and right of the separator, "--". The left integers are for the price of the stock you purchased. The right is the quantity. In this case, we've bought 2 shares at $10 and 4 shares at $11. 

```
Total amount of money spent: $64
Break even average: $10.666667
```
