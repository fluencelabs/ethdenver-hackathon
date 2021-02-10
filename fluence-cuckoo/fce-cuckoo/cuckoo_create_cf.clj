(seq
    (call relay (service "create_cf") ["0"] result)
    (call %init_peer_id% (returnService "run") [result])
)
