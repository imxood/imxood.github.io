package action;

import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.util.List;

import model.Student;

import org.apache.poi.hssf.usermodel.HSSFDataFormat;
import org.apache.poi.hssf.usermodel.HSSFWorkbook;
import org.apache.poi.ss.usermodel.CellStyle;
import org.apache.poi.ss.usermodel.Row;
import org.apache.poi.ss.usermodel.Sheet;
import org.apache.poi.ss.usermodel.Workbook;

import com.opensymphony.xwork2.ActionSupport;


import service.StuService;
import service.StuServiceImpl;

public class ExportStuAction extends ActionSupport {
	/**
	 * 
	 */
	private static final long serialVersionUID = 1L;

	private InputStream excelFile;

	public void ExcelFile() {
		Workbook workbook = null;
		StuService studentService = new StuServiceImpl();
		workbook = new HSSFWorkbook();
		Sheet sheet = workbook.createSheet("学生信息表");
		Row row = sheet.createRow(0);
		row.createCell(0).setCellValue("编号");
		row.createCell(1).setCellValue("姓名");
		row.createCell(2).setCellValue("性别");
		row.createCell(3).setCellValue("年龄");
		row.createCell(4).setCellValue("住址");

		/*
		 * CellStyle cellStyle = workbook.createCellStyle();
		 * cellStyle.setDataFormat(HSSFDataFormat.getBuiltinFormat("m/d/yy"));
		 */
		List<Student> list = studentService.findAll();
		System.out.println("Here");
		System.out.println(list.size());
		for (int i = 1; i <= list.size(); i++) {
			Student stu = list.get(i - 1);
			row = sheet.createRow(i);
			row.createCell(0).setCellValue(stu.getId());
			row.createCell(1).setCellValue(stu.getName());
			row.createCell(2).setCellValue(stu.getGender());
			row.createCell(3).setCellValue(stu.getAge());
			row.createCell(4).setCellValue(stu.getAddr());
		}
		ByteArrayOutputStream baos = new ByteArrayOutputStream();
		try {

			workbook.write(baos);

		} catch (IOException e) {
			e.printStackTrace();
		}
		byte[] aa = baos.toByteArray();
		excelFile = new ByteArrayInputStream(aa, 0, aa.length);
		try {
			baos.close();
		} catch (IOException e) {
			// TODO Auto-generated catch block
			e.printStackTrace();
		}
	}

	public String execute() throws Exception {
		// getExcelFile();
		// super.execute();
		ExcelFile();
		return "suc";
	}

	public InputStream getExcelFile() {
		return excelFile;
	}

}
