import random

# generate a random integer between 1 and 100
random_number = random.randint(1, 100)

user_input = input("Guess an integer between 1 and 100: ")

# convert the user input to an integer
try:
    user_input = int(user_input)
except ValueError:
    print("Please enter a valid number")
    exit()

if user_input < 1 or user_input > 100:
    print("Please enter a number between 1 and 100")
    exit()
elif random_number == user_input:
    print("You guessed correctly!")
else:
    # use f strings to format the string
    print(f"Sorry, the number was {random_number}. Try again next time!")
