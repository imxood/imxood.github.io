package sanitation.dao;

import java.util.List;
/**
 *
 * @param <T>
 */

public interface GenericDAO <T>{
    /**
     * 通过ID获得实体对象
     * 
     * @param id实体对象的标识符
     * @return 该主键值对应的实体对象
     */
    T findById(int id);
    
    /**
     * 将实体对象持久化
     * 
     * @param entity 需要进行持久化操作的实体对象
     * @return 持久化的实体对象
     */
    T makePersitent(T entity); 
    
    /**
     * 将实体变为瞬态
     * 
     * @param entity需要转变为瞬态的实体对象
     */
    void makeTransient(T entity);
    
    /**
     * 将一系列的实体变为瞬态，使用本地sql
     * 
     * @param hql
     */
    void makeTransientByIds(String sql);
    
    /**
     * 
     * 使用hql语句进行分页操作
     * 
     * @param hql    
     * @param offset    第一条记录索引
     * @param pageSize    每页需要显示的记录数
     * @return    查询的记录
     */
    List<T> findByPage(final String hql,final int offset,final int pageSize);
    
    
    /**
     * 使用hql 语句进行分页查询操作
     * 
     * @param hql 需要查询的hql语句
     * @param value 如果hql有一个参数需要传入，value就是传入的参数
     * @param offset 第一条记录索引
     * @param pageSize 每页需要显示的记录数
     * @return 当前页的所有记录
     */
    List<T> findByPage(final String hql , final Object value ,
             final int offset, final int pageSize);
    
    /**
     * 使用hql 语句进行分页查询操作
     * 
     * @param hql 需要查询的hql语句
     * @param values 如果hql有一个参数需要传入，value就是传入的参数
     * @param offset 第一条记录索引
     * @param pageSize 每页需要显示的记录数
     * @return 当前页的所有记录
     */
    List<T> findByPage(final String hql, final Object[] values,
             final int offset, final int pageSize);
    

    /**
     * 使用sql 语句进行分页查询操作
     * 
     * @param sql
     * @param offset
     * @param pageSize
     * @return
     */
    List findByPageSQL(final String sql, 
             final int offset, final int pageSize);
    
    /**
     * 根据语句查找总数
     * @param hql hql语句
     * @return 对应的数目
     */
    Integer getCount(String hql);
    
    
    void updateObj(final String hql,final Object[] values);
}