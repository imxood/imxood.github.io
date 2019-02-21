<%@ page language="java" import="java.util.*" pageEncoding="UTF-8"%>
<%@taglib prefix="s" uri="/struts-tags"%>
<%
	String path = request.getContextPath();
	String basePath = request.getScheme() + "://"
			+ request.getServerName() + ":" + request.getServerPort()
			+ path + "/";
%>

<!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 4.01 Transitional//EN">
<html>
<head>
<base href="<%=basePath%>">

<title>数据管理</title>
</head>

<body>
	<table>
		<td>数据处理</td>
	</table>
	<table border="1" cellspacing="10" bordercolor="#666666">
		<tr>
			<td bgcolor="#E7E7E7">导出Excel</td>
			<td bgcolor="#E7E7E7">导入Excel</td>
		</tr>
		<tr>
			<td><s:a action="export_stu" namespace="/pages">
					<div align="center">信息表导出</div>
				</s:a></td>
			<td><s:a action="import_stu" namespace="/pages">
					<div align="center">信息表导入</div>
				</s:a></td>
		</tr>


	</table>
</body>
</html>