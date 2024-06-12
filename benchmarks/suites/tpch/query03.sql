-- shipping priority query

select l_orderkey, sum(l_extendedprice * (1 - l_discount)) as revenue, o_orderdate, o_shippriority from customer, orders, lineitem where c_mktsegment = ':1' and c_custkey = o_custkey and l_orderkey = o_orderkey and o_orderdate < date ':2' and l_shipdate > date ':2' group by l_orderkey, o_orderdate, o_shippriority order by revenue desc, o_orderdate

-- :1 is pick one of
--  'AUTOMOBILE'
--  'BUILDING'
--  'FURNITURE'
--  'HOUSEHOLD'
--  'MACHINERY'
-- :2 is a date in the format 'YYYY-MM-DD'
--  RandomDate 1995-03-01 .. 1995-03-31
