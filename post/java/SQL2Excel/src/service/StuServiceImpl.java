package service;

import java.util.List;

import model.Student;

import dao.StudentDAO;

public class StuServiceImpl implements StuService {

	public List<Student> findAll() {

		StudentDAO dao = new StudentDAO();
		List<Student> list = dao.findAll();

		return list;
	}

}
