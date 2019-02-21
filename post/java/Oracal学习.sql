
/* 1,出现错误：
	ORA-01034: ORACLE not available 
    ORA-27101: shared memory realm does not exist
  解决办法: */
		

show user;--显示当前
conn username/password as sysdba;(或sysoper)--连接一个用户 拥有数据库dba的角色
disc;			--断开连接
clear screen;	--清屏
start xxxx.sql;	--执行sql代码
@ xxx.sql;		----执行sql代码
desc 表名		--查看一个表的结构
spool  filename --临时的，原来在盘中是不存在的，相当新建一个文件---开始
spool off	 	---结束
edit xxxx.sql 	--编辑sql代码

--用户管理的一些命令:
	--dba具有创建用户的权利,如果让用户可操作其他用户,给他角色--dba
		--创建用户,密码不能以数字开头,刚创建时没有任何权限
			create user username identified by password;
		--添加权限
			--权限对象有:
			dba 管理员权限,拥有数据库的最高权限,全部权限
			connect 数据库的连接权限
			resource 实操权限
			
			创建普通用户：connect resource
			管理员用户
			
			grant 权限对象 [on tablename] to username [with grant option](username可以继续给别人授权);
			
		--修改密码
			(1)PASSWORD username; 
			(2)PASSWORD;
		--修改密码(管理员可以用)
			alter user username identified by password;
		--删除用户
			drop user username [cascade]--(级联删除)	
	
linesize:
	show linesize;	 --查看当前显示的宽度
	set linesize num;--设置当前显示的宽度
		
pagesize
	show linesize;	 --显示每页显示的行数
	set linesize;	 --设置每页显示的行数
		
--数据库中用到的语言分类:
	--数据定义语言(DDL):
		create	alter	drop
	--数据操作语言(DML)
		insert	delete	update 	select
	--事务控制语言(TCL)
		commit(插入)	savepoint(保存点)	rollback(回滚)
	--数据控制语言(DCL)
		grant(授权)		revoke(回收权限)
		
		
--select
	(1)选择操作:查看字段中特定信息
	(2)投影操作:查看特定字段
	(3)连接操作:多表操作
	--别名
		select fieldname "alias" from tablename;
	
	--连接两个字段
		select fieldname1||fieldname2 from tablename;
		select fieldname1||'连接符号'||fieldname2 from tablename;
	
	--两个关键字:
		--把null为指定值
		nvl(fieldname,值) --eg:select nvl(fieldname,0) "age" from users;如果age为null则age为0
		--去重复
		distinct --eg:select distinct fieldname1,fieldname2 from tablename;
		
	--否定表达式
	!=    <>   ^=
	not between  and  --不在两者之间
	is not null --不为空
	is null --是空值
	
	--like
		%表示0或多个字符 --eg:like 's\_%' ,\是转义字符
		
	order by --排序 默认是升序asc,降序是desc
			 --eg:select * from user order by username desc;
	
	--注意: 当要排序字段里含有空值时,升序时，空值在末尾,降序时，空值排在开头
		--(原因:在数据库中存值时,空值表示无穷大)
		--在程序中,如果出现空值,极易产生NullPointException
		
	--字符型:
		varchar2   	--可变长度的字符串	1-4000byte
		char		--固定长度			1-2000byte
		long		--可变长度的字符串	最长可达2gb
		
	--数值类型:
		number		--既可以整数也可以是小数
	--日期类型
		date		--存储日期和时间，精确到秒
		timestamp	--存储日期，，时间，可以精确到小数点后六位
	--大数据对象类型
		blob		--存储二进制对象，一般存储较大的，比如:视频，图片
		bfile		--以二进制的形式存储一些较大的文件
		
	--数据库操作的常用函数
		--(1)字符处理函数
			lower(fieldname) --转换成小写
			upper(fieldname) --转换成大写
			initcap(fieldname) --把首字母转换成大写
			concat(fieldname1,fieldname2) --连接两个字符串
			substr(fieldname,startIndex,length) --截取字符串,startIndex取正则从左向右，取负则从右向左
			
		--在oracle中提供了一个虚表dual 用来测试函数或做运算
		
		--(2)数值函数
			round(num,bit) --四舍五入,bit表示小数点后几位,可为负数,0为各位
			trunc(num,bit) --截取
			
		--(3)日期函数
			--全日制格式:年,月,日,时分秒
			--缺省格式:		日-月-年,dd-mon-rr
			--返回当前日期 sysdate
			--返回明天的日期 sysdate+1
			--十分钟后的日期？？？？？？
			
			months_between(to_date(strDate1,formate),to_date(strDate2,formate));--求月份差
			next_day(to_date(strDate1,formate),"MONDAY"); --下个星期的日期
			last_day(date);--这个月的最后一天
			trunc(日期,)
			
		--(4)转换函数
			--隐式转换，当字符串全是0-9数字时会发生隐式转换
			to_number(str); --字符串转换成数
			to_char(num,format); --数转换成字符串
			
		--(5)组函数
			avg()
			max()
			min()
			sum()
			count()
			stddev() --标准差
			
		--别名：
			(1)加双引号
			(2)直接 无法避免空格 eg:firstname lastname
			(3)as aliasname;
			
			
		having用在group by之后，对分组的数据进行过滤，where是在group by之前
		
		
	--连接
		(1)--内连接
			--满足条件的数据保留,不满足的全部舍弃
		(2)--外连接:
			左外连接(left outer join):确定主表:主表是left outer join前面的那个表,也叫做基表
			右外连接(right outer join):以右边的为主表
			全连接(full outer join):对两个表中的匹配的信息和不匹配的信息都显示出来
			
			无论是左外连接还是右外连接得到的数据数量等于内连接查询到的数据数量加上没有匹配的数量
			
			对于左右连接还有一种表示方式：(+) (—)
				在where条件后面加上(+)或(—):
					eg:select * from tablename1,tablename2 where tablename1.fieldname1 = tablename2.fieldname3(+)
					eg表示tablename1是主表,(+)的对面是主表;
	
	--子查询
		单行查询
		多行查询
		
	SELECT *

	FROM (SELECT ROWNUM AS rowno, t.* FROM emp t WHERE hire_date BETWEEN TO_DATE ('20060501', 'yyyymmdd') AND TO_DATE ('20060731', 'yyyymmdd') AND ROWNUM <= 20)
		table_alias

	WHERE table_alias.rowno >= 10;
	
	
	select * from  
	(select rownum,tablename.* as rown from tablename where rown<20) alias
	where alias.rown>=10;
		
DML select:

		insert into tablename[(fieldnameList)] values(valueList);
		
		update s_dept set region_id=3 where Condition;
		
		delete from tablename where Condition;
		
对数据库中的表的操作
		create table tablename(fieldname varchar2 not null,...);  	--创建表
		alter table tablename add(fieldname type);					--添加列
		alter table tablename drop(fieldname);						--删除列
		
		savepoint ss;
			...
		rollback to ss;
		
		删除表的三种方法
			delete 做删除的时候,把数据unused,数据还在缓冲区中
			truncate table tablename [cascade contraints];直接删掉所有数据(无法rollback,默认有commit;操作)
			drop table tablename;(级联删除)
E-R图
	实体联系图 实体关系图
	
	有哪些实体?
	实体 --一个个体 学生 部门 员工				------矩形
	属性 --学生:学号,姓名,年龄,专业				------椭圆
	联系 --实体与属性之间的联系	实体与实体		------菱形
	
	他们之间用 无向线 连接
	
	有哪些联系:
	一对一
	一对多
	多对多
		--事务:一个完整的执行单元,在两个commit或者rollback之间的操作(DML)成为一个事务
		--前一个事务的结束是另一个事务的开始:
			1,commit:数据库事务提交,将变化写入数据库
			  rollback:数据库事务回退,撤销对数据的修改(rollback;rollback to savepoint_name)
			2,DDL,DCL,一旦发生,事务会自动提交
			3,
			
	主键约束的数据保证不能重复,并且不能为空值---唯一性
	主键可以不只一列
	alter table tablename add primary key(fieldlist...)
	alter table tablename drop primary key(fieldlist...)
	alter table tablename rename constraint old主键名 to new主键名(fieldlist...)
	
	unique唯一约束,可以为空
	
	create table tablename(
		fieldname type .. foreign key(fieldname) references tablename1(fieldname1) no delete [cascade]||[set null]
	)
	
	no delete --设定外键所关联的主键被删除时,关联的外键是否被删除,如果用cascade,则级联删除,若set null删除后外键为null
	
	注意:外键引用不了主键
	
	ORA-02270:此列列表的唯一或主键不匹配
	
	检查约束 除了可以插入约束条件内的数据,还可以插入null
	
	alter table tablename add constraint 约束名 check(fieldname 运算符 值)
	
	--创建索引
	create [unique] index indexname on tablename(fieldlist);
	
	create [or replace] [force 或者 noforce] view viewname [aliasname] as 查询语句
	
	or replace:如果视图已经存在,oracle会重新创建该视图
	
plSQL 是一门过程化语言
	块------>过程		函数------>包(包头,包体)
	
数据的处理
	语句操作:
		if then else end if;
		loop    end loop;
	
	编程分为三个部分:
		定义部分,执行部分,例外部分
		
		declare --定义
		
		create [or replace] procedure procInsert [name in[or out] varchar(20)] is --in/out 输入/输出
		begin
			insert ....
		end
		
		exec procInsert;--执行
		
		show error;--显示错误
		
		name varchar2(20):='maxu'; ----定义常量
		
		
	update tablename set fieldname=value;
	delete from tablename where fieldname=value;
	
