from accounts import Account

account1 = Account(100.0)
account2 = Account(100.0)

account1.deposit(50.0)

account2.withdraw(25.0)

print("balance of account 1:", account1.get_balance())
print("balance of account 2:", account2.get_balance())
