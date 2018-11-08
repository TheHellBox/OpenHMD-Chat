physic_test = World.CreateGameObject()
physic_test:with_model("cube")
physic_test:with_position(0, -3, 0)
physic_test:with_scale(0.01, 0.1, 0.1)
physic_test = physic_test:build()

collider = World.CreateCollider()
collider:WithSize(0.01, 0.1, 0.1)
collider = collider:Build()

physic_test:AttachCollider(collider)


cube = World.CreateGameObject()
cube:with_model("cube")
cube = cube:build()

collider = World.CreateCollider()
collider:Static(false)
collider = collider:Build()

cube:AttachCollider(collider)
cube:SetPosition(0, 5, 0)
