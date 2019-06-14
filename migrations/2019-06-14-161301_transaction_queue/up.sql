-- Your SQL goes here
CREATE VIEW transaction_queue_views (
    type_name,
    type_id,
    character_id,    
    buy_transaction_id,    
    buy_date,    
    quantity,
    buy_unit_price
) AS 
    SELECT
        it.type_name,
        it.type_id,
        tq.character_id,
        tq.transaction_id,
        wt.date,
        tq.amount_left,
        wt.unit_price
    FROM transaction_queues AS tq
    LEFT JOIN wallet_transactions AS wt ON wt.transaction_id = tq.transaction_id
    LEFT JOIN inv_types AS it ON it.type_id = tq.type_id;
