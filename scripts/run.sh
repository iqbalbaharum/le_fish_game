npx aqua run \
  --input ./aqua/game.aqua --func 'test()' \
  --addr /dns4/fluence.xfero.io/tcp/9990/ws/p2p/12D3KooWHBG9oaVx4i3vi6c1rSBUm7MLBmyGmmbHoZ23pmjDCnvK \
  --timeout 100000

# npx aqua run --input ./aqua/game.aqua --func 'test2([{"data": "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001","topics":["0x696e7d567fdcab2406e8de617caf46be048c7600c884e97b9f050754ffa27143"],"transaction_hash": "0xab964e945338426a5bf804f599e07e8f04bac050abdb5e162bf06fb332b7520b"}])' --addr /dns4/fluence.xfero.io/tcp/9990/ws/p2p/12D3KooWHBG9oaVx4i3vi6c1rSBUm7MLBmyGmmbHoZ23pmjDCnvK