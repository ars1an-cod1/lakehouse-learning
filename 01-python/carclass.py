class Car: 
    def __init__(self,brand,km):

        self.brand=brand
        self.km=km
        self.is_sold=False # We dont add inside of the method for not writing everytime False


    def sell (self):
        if self.is_sold:#Has it solded ? --> If it is true-->
            print(f"{self.brand} is already sold ")

        else: # Has not it solded? --> if it is false we gon' sell now 
            self.is_sold=True
            print(f"{self.brand} is not available for sold ")

    def status(self):
        if self.is_sold==False:
            print(f"{self.brand} is available for sale ")
        else:
             print(f"{self.brand} is sold")

        

    def drive(self,distance):
        self.km=distance+self.km
        print(f"{self.brand} now has{self.km} km")

                


bmw = Car("BMW", 45000)
bmw.status() ## before selling --> available for sale
bmw.sell()
bmw.sell()  
bmw.status() ## after selling--> sold
bmw.drive(200)
bmw.drive(50)      



            

        