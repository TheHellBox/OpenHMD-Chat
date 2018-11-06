local players = {}


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

    player_parts[1] = player_head
    player_parts[2] = player_body
    player_parts[3] = player_hat

    table.insert(players, {player_parts, player})

    player:SendLua([===[
        print("Welcome to server!")
        World.LoadModel("./assets/models/default/head.obj", "player_head")
        World.LoadModel("./assets/models/default/body.obj", "player_body")
        World.LoadModel("./assets/models/default/hat.obj", "player_hat")

        local player_hat = World.GetGameObject("player_hat"..connection_id())
        player_hat:SetModel("None")

        local player_head = World.GetGameObject("player_head"..connection_id())
        player_head:SetModel("None")

        local player_body = World.GetGameObject("player_body"..connection_id())
        player_body:SetModel("None")
    ]===])

end)

AddEvent("Update", "update_players_position", function()
    for k, v in pairs(players) do
        local net_player = v[2]

        local player_head = v[1][1]
        local player_body = v[1][2]
        local player_hat = v[1][3]

        local pos = net_player:GetPosition()
        local rot = net_player:GetRotation()

        player_head:SetPosition(pos[1], pos[2] + 0.03, pos[3])
        player_head:SetRotation(rot[1], rot[2], rot[3])

        -- To avoid gimbal lock problems
        local dir = player_head:Direction(0, 0, 1)

        player_body:SetPosition(pos[1], pos[2] - 0.2, pos[3])
        player_body:LookAt(-dir[1], 0.0, dir[3])

        player_hat:SetPosition(pos[1], pos[2] + 0.05, pos[3])
        player_hat:SetRotation(rot[1], rot[2], rot[3])
    end
end)
