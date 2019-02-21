1,IO操作：









2,线程操作:











3,property文件的配置：

import java.io.FileInputStream;
import java.io.FileNotFoundException;
import java.io.FileOutputStream;
import java.io.IOException;
import java.io.UnsupportedEncodingException;
import java.util.Date;
import java.util.Map;
import java.util.Properties;

/**
 * @author zxw
 * 
 */
public class GetSaveProp {

	private void saveProperty() {
		// 保存文件
		Properties propertie = new Properties();
		String characterString = "1中国的";
		propertie.setProperty("character", characterString);
		propertie.setProperty("date", new Date().toString());

		String fileName = "savetest.properties";
		String description = "CharaterTest";
		try {
			FileOutputStream outputFile = new FileOutputStream(fileName);
			propertie.store(outputFile, description);// property类关键的store方法
			outputFile.close();
			// propertie.list(System.out);
			System.out.println("File was saved!");
		} catch (FileNotFoundException e) {
			e.printStackTrace();
		} catch (IOException ioe) {
			ioe.printStackTrace();
		}
	}

	public static void main(String[] args) {
		new GetSaveProp().saveProperty();//save file

		// read from property
		Properties readProps = new Properties();
		FileInputStream inStream;
		try {
			inStream = new FileInputStream("savetest.properties");
			readProps.load(inStream);
			// props.list(System.out);
			if (readProps.get("character") != null) {
				System.out.println("character="
						+ new String(readProps.getProperty("character")
								.getBytes("ISO-8859-1"), "UTF-8"));
				System.out.println("character="
						+ new String(readProps.getProperty("character")
								.getBytes("UTF-8"), "UTF-8"));

			} else {
				System.out.println(readProps.get("character"));
			}
		} catch (FileNotFoundException e) {
			e.printStackTrace();
		} catch (IOException e) {
			e.printStackTrace();
		}
	}
}







4,HashMap与HashTable




5,什么是java序列化，如何实现java序列化？

Java 串行化技术可以使你将一个对象的状态写入一个Byte 流里，
并且可以从其它地方把该Byte 流里的数据读出来，重新构造一个相同的对象.
这种机制允许你将对象通过网络进行传播，并可以随时把对象持久化到数据库、文件等系统里。
Java的串行化机制是RMI、EJB等技术的技术基础。
用途：利用对象的串行化实现保存应用程序的当前工作状态，下次再启动的时候将自动地恢复到上次执行的状态。

http://blog.csdn.net/yakihappy/article/details/3979373


6,oracal的安装


HKEY_LOCAL_MACHINE		
		
		SOFTWARE
			ORACAL(删掉)
		SYSTEM
			ORACAL(以这个开头的全删除)
			CURRENTVONTROL_SET

7,时间转换:
SimpleDateFormat df = new SimpleDateFormat("yyyy-MM-dd HH:mm:ss");//设置日期格式
System.out.println(df.format(new Date()));// new Date()为获取当前系统时间



create trigger triggerName

after/before insert/update/delete on 表名

for each row   #这句话在mysql是固定的

begin

sql语句;

end;