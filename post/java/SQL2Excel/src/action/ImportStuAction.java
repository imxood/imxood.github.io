package action;

import java.io.FileInputStream;
import java.io.FileNotFoundException;
import java.io.IOException;
import java.io.InputStream;
import java.sql.Connection;
import java.sql.PreparedStatement;
import java.sql.SQLException;
import org.apache.poi.hssf.usermodel.HSSFCell;
import org.apache.poi.hssf.usermodel.HSSFRow;
import org.apache.poi.hssf.usermodel.HSSFSheet;
import org.apache.poi.hssf.usermodel.HSSFWorkbook;

import utils.DBConnection;

public class ImportStuAction {

	private Connection con;
	private DBConnection db;
	private PreparedStatement pst;
	// excel所在路径
	private String filePath = "/excel/stu.xls";
	public FileInputStream fis;

	public boolean insertDB() {
		System.out.println("import");
		boolean flag = true;
		db = new DBConnection();
		con = db.getConnection();
		try {
			// 文件流指向excel文件
			InputStream resourceAsStream = ImportStuAction.class
					.getResourceAsStream(filePath);
			HSSFWorkbook workbook = new HSSFWorkbook(resourceAsStream);// 创建工作薄
			HSSFSheet sheet = workbook.getSheetAt(0);// 得到工作表
			HSSFRow row = null;// 对应excel的行
			HSSFCell cell = null;// 对应excel的列

			int totalRow = sheet.getLastRowNum();// 得到excel的总记录条数
			System.out.println("总行数为:" + totalRow);

			// 以下的字段一一对应数据库表的字段

			// int perreqcno;
			String id = "";
			String name = "";
			String gender = "";
			int age;
			String addr = "";

			String sql = "insert into student(id, name, gender, age, addr) values(?,?,?,?,?)";

			for (int i = 1; i <= totalRow; i++) {
				row = sheet.getRow(i);

				/*
				 * cell = row.getCell((short) 1); perreqcno = (int)
				 * cell.getNumericCellValue();// 第二字段为action_name，故转为String类型
				 * System.out.println("action_name is" + perreqcno);
				 */
				cell = row.getCell((short)0);
				id = cell.toString();
				cell = row.getCell((short)1);
				name = cell.toString();
				cell = row.getCell((short) 2);
				gender = cell.toString();
				cell = row.getCell((short) 3);
				System.out.println(cell.toString());
				age = (int)Double.parseDouble(cell.toString());
//				age = (int) cell.getNumericCellValue();
				cell = row.getCell((short) 4);
				addr = cell.toString();

				pst = con.prepareStatement(sql);
				pst.setString(1, id);
				pst.setString(2, name);
				pst.setString(3, gender);
				pst.setInt(4, age);
				pst.setString(5, addr);
//				pst.execute();
				System.out.println("Here");
				pst.executeUpdate();
			}
		} catch (FileNotFoundException e) {
			flag = false;
			e.printStackTrace();
		} catch (IOException ex) {
			flag = false;
			ex.printStackTrace();
		} catch (SQLException exx) {
			flag = false;
			exx.printStackTrace();
		}
		return flag;
	}

	public String execute() throws Exception {
		// getExcelFile();
		// super.execute();
		ImportStuAction iua = new ImportStuAction();
		insertDB();
		return "suc";
	}
}