public LinkedHashMap<String, List> getExportData()
{
    LinkedHashMap<String, List> map = new LinkedHashMap<String, List>();
    List<Role> roles = roleMapper.selectByCriteria(null);
    List<User> dataList = new ArrayList<User>();
    for(User user : findAll())
    {
        if(user.getType() == Constants.UserType.USERTYPE_ADMINISTRATOR)
            user.setTypeStr(Constants.UserType.USERTYPE_ADMINISTRATOR_STR);
        if(user.getType() == Constants.UserType.USERTYPE_PERSONAL)
            user.setTypeStr(Constants.UserType.USERTYPE_PERSONAL_STR);
        if(user.getType() == Constants.UserType.USERTYPE_COMPANY)
            user.setTypeStr(Constants.UserType.USERTYPE_COMPANY_STR);
         
        for(Role role : roles)
        {
            if(user.getRoleId() == role.getId())
            {
                user.setRoleStr(role.getRoleName());
                break;
            }
        }
         
        dataList.add(user);
    }
    map.put("后台用户信息", dataList);
    return map;
}