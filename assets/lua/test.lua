btn1 = Ui.AddButton("hey", 100, 100, "button_pressed_1")
btn1:SetSize(128, 300)
btn1:SetLabel("Not a 'hey' anymore")

Ui.AddButton("hey2", 300, 100, "button_pressed_2")

AddEvent("button_pressed_1", "print_something", function()
	print("Hello there")
end)

AddEvent("button_pressed_2", "print_something2", function()
	print("Hello there 2")
end)
