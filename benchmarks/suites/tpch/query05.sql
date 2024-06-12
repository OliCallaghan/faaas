-- this query lists the revenue volume done through local suppliers.

select n_name, sum(l_extendedprice * (1 - l_discount)) as revenue from customer, orders, lineitem, supplier, nation, region where c_custkey = o_custkey and l_orderkey = o_orderkey and l_suppkey = s_suppkey and c_nationkey = s_nationkey and s_nationkey = n_nationkey and n_regionkey = r_regionkey and r_name = ':1' and o_orderdate >= date ':2' and o_orderdate < date ':2' + interval '1 year' group by n_name order by revenue desc

-- :1. REGION is randomly selected within the list of values defined for R_NAME in Clause 4.2.3;
--   'AFRICA'
--   'AMERICA'
--   'ASIA'
--   'EUROPE'
--   'MIDDLE EAST'
-- :2. DATE is the first of January of a randomly selected year within [1993 .. 1997].
