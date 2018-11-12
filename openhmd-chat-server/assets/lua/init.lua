local players = {}

function tablelength(T)
  local count = 0
  for _ in pairs(T) do count = count + 1 end
  return count
end

function simple_lerp(a, b, weight)
    return a + weight * (b-a)
end

AddEvent("OnClientConnected", "init_player", function(id)
    player = Player.GetByID(id)

    player_parts = {}

    player_head = World.CreateGameObject()
    player_head:with_name("player_head"..id)
    player_head:with_model("player_head")
    player_head:with_position(0, 0, 0)
    player_head = player_head:build()

    player_body = World.CreateGameObject()
    player_body:with_name("player_body"..id)
    player_body:with_model("player_body")
    player_body:with_position(0, 0, 0)
    player_body = player_body:build()

    player_hat = World.CreateGameObject()
    player_hat:with_name("player_hat"..id)
    player_hat:with_model("player_hat")
    player_hat:with_position(0, 0, 0)
    player_hat = player_hat:build()

    collider = World.CreateCollider()
    collider:WithSize(0.1, 0.5, 0.1)
    collider = collider:Build()
    player_body:AttachCollider(collider)

    player_parts[1] = player_head
    player_parts[2] = player_body
    player_parts[3] = player_hat

    table.insert(players, {player_parts, player})

    player:SendLua([===[
        print("Welcome to server!")
        World.LoadModel("./assets/models/default/head.obj", "player_head")
        World.LoadModel("./assets/models/default/body.obj", "player_body")
        World.LoadModel("./assets/models/default/hat.obj", "player_hat")
        World.LoadModel("./assets/models/cube/cube.obj", "cube")

        local player_hat = World.GetGameObject("player_hat"..connection_id())
        player_hat:SetModel("None")

        local player_head = World.GetGameObject("player_head"..connection_id())
        player_head:SetModel("None")

        local player_body = World.GetGameObject("player_body"..connection_id())
        player_body:SetModel("None")
    ]===])
end)

AddEvent("OnClientDisconnected", "remove_player", function(id)
    print("disconected")
    for k, v in pairs(players) do
        if v[2]:Id() == id then
            for k, v in pairs(v[1]) do
                v:Remove()
            end
            table.remove(players, k)
        end
    end
end)

AddEvent("Update", "update_players_position", function()

end)
