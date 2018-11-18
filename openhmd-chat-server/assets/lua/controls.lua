local players_rotation = {}
AddEvent("OnClientConnected", "init_player_controls", function(id)
  player = Player.GetByID(id)
  players_rotation[id] = {player, 0}
end)

AddEvent("OnKeyboardInput", "on_key_input", function(id, key, pressed)
  if pressed == true then return end
  local player = players_rotation[id][1]
  if key == 17 then
    local position = player:GetPosition()
    local direction = player:Direction(0, 0, -1)
    player:SetPosition(position[1] + direction[1] / 2, position[2] + direction[2] / 2, position[3] + direction[3] / 2)
  end
  if key == 31 then
    local position = player:GetPosition()
    local direction = player:Direction(0, 0, 1)
    player:SetPosition(position[1] + direction[1] / 2, position[2] + direction[2] / 2, position[3] + direction[3] / 2)
  end
  if key == 30 then
    local position = player:GetPosition()
    local direction = player:Direction(-1, 0, 0)
    player:SetPosition(position[1] + direction[1] / 2, position[2] + direction[2] / 2, position[3] + direction[3] / 2)
  end
  if key == 32 then
    local position = player:GetPosition()
    local direction = player:Direction(1, 0, 0)
    player:SetPosition(position[1] + direction[1] / 2, position[2] + direction[2] / 2, position[3] + direction[3] / 2)
  end

  if key == 16 then
    players_rotation[id][2] = players_rotation[id][2] - 3.14 / 4
    if players_rotation[id][2] > 3.14 then
      players_rotation[id][2] = 0
    end
    player:SetRotation(0, players_rotation[id][2], 0)
  end
  if key == 18 then
    players_rotation[id][2] = players_rotation[id][2] + 3.14 / 4
    if players_rotation[id][2] > 3.14 then
      players_rotation[id][2] = 0
    end
    player:SetRotation(0, players_rotation[id][2], 0)
  end
end)
