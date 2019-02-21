package model;

import dao.AbstractStudent;




/**
 * Student entity. @author MyEclipse Persistence Tools
 */
public class Student extends AbstractStudent implements java.io.Serializable {

    // Constructors

    /** default constructor */
    public Student() {
    }

    
    /** full constructor */
    public Student(String name, String gender, Integer age, String addr) {
        super(name, gender, age, addr);        
    }
   
}
