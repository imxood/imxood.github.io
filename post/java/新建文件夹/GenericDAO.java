package sanitation.dao;

import java.util.List;
/**
 *
 * @param <T>
 */

public interface GenericDAO <T>{
    /**
     * ͨ��ID���ʵ�����
     * 
     * @param idʵ�����ı�ʶ��
     * @return ������ֵ��Ӧ��ʵ�����
     */
    T findById(int id);
    
    /**
     * ��ʵ�����־û�
     * 
     * @param entity ��Ҫ���г־û�������ʵ�����
     * @return �־û���ʵ�����
     */
    T makePersitent(T entity); 
    
    /**
     * ��ʵ���Ϊ˲̬
     * 
     * @param entity��Ҫת��Ϊ˲̬��ʵ�����
     */
    void makeTransient(T entity);
    
    /**
     * ��һϵ�е�ʵ���Ϊ˲̬��ʹ�ñ���sql
     * 
     * @param hql
     */
    void makeTransientByIds(String sql);
    
    /**
     * 
     * ʹ��hql�����з�ҳ����
     * 
     * @param hql    
     * @param offset    ��һ����¼����
     * @param pageSize    ÿҳ��Ҫ��ʾ�ļ�¼��
     * @return    ��ѯ�ļ�¼
     */
    List<T> findByPage(final String hql,final int offset,final int pageSize);
    
    
    /**
     * ʹ��hql �����з�ҳ��ѯ����
     * 
     * @param hql ��Ҫ��ѯ��hql���
     * @param value ���hql��һ��������Ҫ���룬value���Ǵ���Ĳ���
     * @param offset ��һ����¼����
     * @param pageSize ÿҳ��Ҫ��ʾ�ļ�¼��
     * @return ��ǰҳ�����м�¼
     */
    List<T> findByPage(final String hql , final Object value ,
             final int offset, final int pageSize);
    
    /**
     * ʹ��hql �����з�ҳ��ѯ����
     * 
     * @param hql ��Ҫ��ѯ��hql���
     * @param values ���hql��һ��������Ҫ���룬value���Ǵ���Ĳ���
     * @param offset ��һ����¼����
     * @param pageSize ÿҳ��Ҫ��ʾ�ļ�¼��
     * @return ��ǰҳ�����м�¼
     */
    List<T> findByPage(final String hql, final Object[] values,
             final int offset, final int pageSize);
    

    /**
     * ʹ��sql �����з�ҳ��ѯ����
     * 
     * @param sql
     * @param offset
     * @param pageSize
     * @return
     */
    List findByPageSQL(final String sql, 
             final int offset, final int pageSize);
    
    /**
     * ��������������
     * @param hql hql���
     * @return ��Ӧ����Ŀ
     */
    Integer getCount(String hql);
    
    
    void updateObj(final String hql,final Object[] values);
}