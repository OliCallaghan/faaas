-- minimum cost supplier query

select s_acctbal, s_name, n_name, p_partkey, p_mfgr, s_address, s_phone, s_comment from part, supplier, partsupp, nation, region where p_partkey = ps_partkey and s_suppkey = ps_suppkey and p_size = :1 and p_type like '%:2' and s_nationkey = n_nationkey and n_regionkey = r_regionkey and r_name = ':3' and ps_supplycost = ( select min(ps_supplycost) from partsupp, supplier, nation, region where p_partkey = ps_partkey and s_suppkey = ps_suppkey and s_nationkey = n_nationkey and n_regionkey = r_regionkey and r_name = ':3') order by s_acctbal desc, n_name, s_name, p_partkey

-- :1 is RandomNumber 1 and 50
-- :2 is pick one of
--   'TIN'
--   'NICKEL'
--   'BRASS'
--   'STEEL'
--   'COPPER'
-- :3 is pick one of
--   'AFRICA'
--   'AMERICA'
--   'ASIA'
--   'EUROPE'
--   'MIDDLE EAST'
