btn1 = Ui.add_button("hey", 100, 100, "button_pressed_1")
btn1:set_size(128, 300)
Ui.add_button("hey2", 300, 100, "button_pressed_2")

AddEvent("button_pressed_1", "print_something", function()
	print("Hello there")
end)

AddEvent("button_pressed_2", "print_something2", function()
	print("Hello there 2")
end)

