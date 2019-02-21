public InputStream getUserExcel() throws 
    IllegalArgumentException, IOException, IllegalAccessException
{
    ByteArrayOutputStream baos = new ByteArrayOutputStream();
    List<String[]> headNames = new ArrayList<String[]>();
    headNames.add(new String[] { "用户名", "密码", "电子邮件", "类型", "角色" });
    List<String[]> fieldNames = new ArrayList<String[]>();
    fieldNames.add(new String[] { "userName", "pwd", "email", "typeStr", "roleStr"});
 
    ExportSetInfo setInfo = new ExportSetInfo();
    setInfo.setObjsMap(userService.getExportData());
    setInfo.setFieldNames(fieldNames);
    setInfo.setTitles(new String[] { "馋八戒后台用户信息" });
    setInfo.setHeadNames(headNames);
    setInfo.setOut(baos);
     
    // 将需要导出的数据输出到baos
    ExcelUtil.export2Excel(setInfo);
     
    return new ByteArrayInputStream(baos.toByteArray());
}