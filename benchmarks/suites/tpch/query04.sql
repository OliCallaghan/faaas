-- This query determines how well the order priority system is working and gives an assessment of customer satisfac-tion.

select o_orderpriority, count(*) as order_count from orders where o_orderdate >= date ':1' and o_orderdate < date ':1' + interval '3 month' and exists ( select * from lineitem where l_orderkey = o_orderkey and l_commitdate < l_receiptdate) group by o_orderpriority order by o_orderpriority

-- :1 is any first of the month between 1993-01-01 and 1997-10-01
--   eg. 1993-07-01 (has to be a first of the month)
