package sanitation.dao.impl;

import java.sql.SQLException;
import java.util.List;

import org.hibernate.HibernateException;
import org.hibernate.Query;
import org.hibernate.Session;
import org.springframework.orm.hibernate3.HibernateCallback;
import org.springframework.orm.hibernate3.support.HibernateDaoSupport;

import sanitation.dao.GenericDAO;

public class GenericHibernateDAO<T> extends HibernateDaoSupport 
    implements GenericDAO<T>{

    private Class<T> persistentClass;
    
    public GenericHibernateDAO(Class<T> persistentClass){
        this.persistentClass=persistentClass;
    }
    
    public Class<T> getPersistentClass(){
        return persistentClass;
    }
    
    @SuppressWarnings("unchecked")
    public T findById(int id) {
        return (T)getHibernateTemplate().get(getPersistentClass(), id);
    }

    @SuppressWarnings("unchecked")
    public List<T> findByPage(final String hql, 
         final int offset, final int pageSize){
        List<T> list= getHibernateTemplate().executeFind(new HibernateCallback(){
                public Object doInHibernate(final Session session)
                    throws HibernateException, SQLException{
                    Query query=session.createQuery(hql);
                    if(!(offset==0 && pageSize==0)){
                        query.setFirstResult(offset).setMaxResults(pageSize);
                    }
                    List<T> result = query.list();
                    return result;
                }
            });
        return list;
    }
    
    @SuppressWarnings("unchecked")
    public List findByPageSQL(final String sql, 
         final int offset, final int pageSize){
        List list= getHibernateTemplate().executeFind(new HibernateCallback(){
                public Object doInHibernate(final Session session)
                    throws HibernateException, SQLException{
                    Query query=session.createSQLQuery(sql);
                    if(!(offset==0 && pageSize==0)){
                        query.setFirstResult(offset).setMaxResults(pageSize);
                    }
                    List result = query.list();
                    return result;
                }
            });
        return list;
    }
    

    @SuppressWarnings("unchecked")
    public List<T> findByPage(final String hql, final Object value, 
            final int offset, final int pageSize) {
        List<T> list = getHibernateTemplate().executeFind(new HibernateCallback()
        {
            public Object doInHibernate(Session session)
                throws HibernateException, SQLException
            {
                Query query=session.createQuery(hql).setParameter(0, value);
                if(!(offset==0 && pageSize==0)){
                    query.setFirstResult(offset).setMaxResults(pageSize);
                }
                List<T> result = query.list();
                return result;
            }
        });
        return list;
    }

    @SuppressWarnings("unchecked")
    public List<T> findByPage(final String hql, final Object[] values, final int offset,
            final int pageSize) {
        List<T> list = getHibernateTemplate().executeFind(new HibernateCallback(){
            public Object doInHibernate(Session session)
                throws HibernateException, SQLException{
                Query query=session.createQuery(hql);
                for (int i = 0 ; i < values.length ; i++){
                    query.setParameter( i, values[i]);
                }
                if(!(offset==0 && pageSize==0)){
                    query.setFirstResult(offset).setMaxResults(pageSize);
                }
                List<T> result = query.list();
                return result;
            }
        });
    return list;
    }
    

    public void updateObj(final String hql, final Object[] values) {
        getHibernateTemplate().execute(new HibernateCallback(){
            public Object doInHibernate(Session session)
            throws HibernateException, SQLException{
                Query query=session.createQuery(hql);
                for(int i=0;i<values.length;i++){
                    query.setParameter( i, values[i]);
                }
                query.executeUpdate();
                return null;    
            }
        });
    }

    public Integer getCount(String hql) {
        Integer count;
        //iterate方法与list方法的区别是list取出全部，iterator取出主键，迭代的时候才取出数据
        count = ((Long)getHibernateTemplate().iterate(hql).next()).intValue();
        return count;
    }

    public T makePersitent(T entity) {
        getHibernateTemplate().saveOrUpdate(entity);
        return entity;
    }

    public void makeTransient(T entity) {
        getHibernateTemplate().delete(entity);
    }

    public void makeTransientByIds(final String sql) {
        getHibernateTemplate().execute(new HibernateCallback(){
            public Object doInHibernate(Session session)
            throws HibernateException, SQLException{
                Query query=session.createQuery(sql);
                query.executeUpdate();
                return null;
            }
        });
    }

}