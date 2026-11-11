#Creating Dictionary 
grades={
    "math":80, "physics": 57 , "Computer Science": 89 , 
}
total=sum(grades.values())
count=len(grades)
avarage=total/count
grades["chemistry"]=100
grades["physics"]=70
grades.get("biology")
print(grades.get("biology","not entered"))
grades.get("math")#it finds the value but doesn't demonstrate 
print(grades.get("math"))# It prints the value on terminal
                        #JSON is also dict  key--> value
                        #Config files
                        #Rust HashMap
                        #PySpark
                        #Counting and Grouping
                        #On here keys are our subject names // Value(every subject's grade is our values)
total=sum(grades.values())
count=len(grades)
avarage=total/count     
print(avarage)                   #grades is neither a key or value. It's the name of the whole dict the name of
for subject,grade in grades.items(): # .items is not variable which is we have not created 
    
    print(f"{subject}:{grade}")  
car={
    "brand": "BMW", "km": 42157
}

car["color"]="blue"# adds a new key-value pair
car["km"]=57000 #updates to existing value of km

print(car)

for key,value in car.items():
    print(f"{key}:{value}")

