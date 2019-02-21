package dao;
// default package



/**
 * AbstractStudent entity provides the base persistence definition of the Student entity. @author MyEclipse Persistence Tools
 */

public abstract class AbstractStudent  implements java.io.Serializable {


    // Fields    

     private String id;
     private String name;
     private String gender;
     private Integer age;
     private String addr;


    // Constructors

    /** default constructor */
    public AbstractStudent() {
    }

    
    /** full constructor */
    public AbstractStudent(String name, String gender, Integer age, String addr) {
        this.name = name;
        this.gender = gender;
        this.age = age;
        this.addr = addr;
    }

   
    // Property accessors

    public String getId() {
        return this.id;
    }
    
    public void setId(String id) {
        this.id = id;
    }

    public String getName() {
        return this.name;
    }
    
    public void setName(String name) {
        this.name = name;
    }

    public String getGender() {
        return this.gender;
    }
    
    public void setGender(String gender) {
        this.gender = gender;
    }

    public Integer getAge() {
        return this.age;
    }
    
    public void setAge(Integer age) {
        this.age = age;
    }

    public String getAddr() {
        return this.addr;
    }
    
    public void setAddr(String addr) {
        this.addr = addr;
    }
   
}