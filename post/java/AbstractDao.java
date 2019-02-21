/*
 * Copyright (c) 2012 DongXiangli.All rights reserved.
 */
 
package com.codeworker.utils.dao;
 
import org.hibernate.Criteria;
import org.hibernate.Query;
import org.hibernate.Session;
import org.hibernate.SessionFactory;
import org.hibernate.criterion.*;
import org.hibernate.internal.CriteriaImpl;
import org.hibernate.transform.ResultTransformer;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.util.Assert;
 
import java.io.Serializable;
import java.util.ArrayList;
import java.util.List;
 
/**
 * User: DongXiangli
 * Date: 12-12-24
 * Time: ����10:44
 */
public class AbstractDao<T, PK extends Serializable> {
 
    protected final Logger logger = LoggerFactory.getLogger(this.getClass());
    protected Class<T> entityClass;
    private SessionFactory sessionFactory;
 
    public AbstractDao(SessionFactory sessionFactory, Class<T> entityClass) {
        this.sessionFactory = sessionFactory;
        this.entityClass = entityClass;
    }
 
    public Session getSession() {
        return sessionFactory.getCurrentSession();
    }
 
    public SessionFactory getSessionFactory() {
        return sessionFactory;
    }
 
    public void save(T entity) {
        Assert.notNull(entity);
        getSession().saveOrUpdate(entity);
        logger.info("save entity: {}", entity);
    }
 
    public void delete(T entity) {
        Assert.notNull(entity);
        getSession().delete(entity);
        logger.info("delete entity: {}", entity);
    }
 
    public void delete(PK id) {
        Assert.notNull(id);
        getSession().delete(getSession().load(entityClass, id));
    }
 
    public int delete(PK[] ids) {
        Assert.notNull(ids);
        int i = 0;
        for (PK id : ids) {
            delete(id);
            i++;
        }
        return i;
    }
 
    public List<T> findAll() {
        return findByCriteria();
    }
 
    public PageUtils<T> findAll(PageUtils<T> page) {
        return findByCriteria(page);
    }
 
    /**
     * ��id��ȡ����.
     */
    public T get(final PK id) {
        Assert.notNull(id);
        return (T) getSession().get(entityClass, id);
    }
 
    /**
     * ��HQL��ѯ�����б�.
     *
     * @param hql    hql���
     * @param values �����ɱ�Ĳ���
     */
    public List<T> find(String hql, Object... values) {
        return createQuery(hql, values).list();
    }
 
    /**
     * ��HQL��ҳ��ѯ.
     * �ݲ�֧���Զ���ȡ�ܽ����,���û�����ִ�в�ѯ.
     *
     * @param page   ��ҳ����.����pageSize ��firstResult.
     * @param hql    hql���.
     * @param values �����ɱ�Ĳ���.
     * @return ��ҳ��ѯ���, ��������б����в�ѯʱ�Ĳ���.
     */
    public PageUtils<T> find(PageUtils<T> page, String hql, Object... values) {
        Assert.notNull(page);
 
        if (page.isAutoCount()) {
            logger.warn("HQL��ѯ�ݲ�֧���Զ���ȡ�ܽ����,hqlΪ{}", hql);
        }
        Query q = createQuery(hql, values);
        if (page.isFirstSetted()) {
            q.setFirstResult(page.getFirst());
        }
        if (page.isPageSizeSetted()) {
            q.setMaxResults(page.getPageSize());
        }
        page.setData(q.list());
        return page;
    }
 
    //����ָ�������Ĳ�ѯ���
    public List<T> findByCount(int iCount, String hql, Object... values) {
        Query q = createQuery(hql, values);
        q.setMaxResults(iCount);
        return q.list();
    }
 
    /**
     * ��HQL��ѯΨһ����.
     */
    public Object findUnique(String hql, Object... values) {
        return createQuery(hql, values).uniqueResult();
    }
 
    /**
     * ��HQL��ѯIntger���ν��.
     */
    public Integer findInt(String hql, Object... values) {
        return ((Long) findUnique(hql, values)).intValue();
    }
 
    /**
     * ��HQL��ѯLong���ͽ��.
     */
    public Long findLong(String hql, Object... values) {
        return (Long) findUnique(hql, values);
    }
 
    /**
     * ��Criterion��ѯ�����б�.
     *
     * @param criterion �����ɱ��Criterion.
     */
    public List<T> findByCriteria(Criterion... criterion) {
        return createCriteria(criterion).list();
    }
 
    /**
     * ��Criterion��ҳ��ѯ.
     *
     * @param page      ��ҳ����.����pageSize��firstResult��orderBy��asc��autoCount.
     *                  ����firstResult��ֱ��ָ��,Ҳ����ָ��pageNo.
     *                  autoCountָ���Ƿ�̬��ȡ�ܽ����.
     * @param criterion �����ɱ��Criterion.
     * @return ��ҳ��ѯ���.��������б����в�ѯʱ�Ĳ���.
     */
    public PageUtils<T> findByCriteria(PageUtils<T> page, Criterion... criterion) {
        Assert.notNull(page);
 
        Criteria c = createCriteria(criterion);
        c.setResultTransformer(CriteriaSpecification.DISTINCT_ROOT_ENTITY);
 
        if (page.isAutoCount()) {
            page.setTotalCount(countQueryResult(page, c));
        }
        if (page.isFirstSetted()) {
            c.setFirstResult(page.getFirst());
        }
        if (page.isPageSizeSetted()) {
            c.setMaxResults(page.getPageSize());
        }
 
        if (page.isOrderBySetted()) {
            if (page.getOrder().endsWith(PageUtils.ASC)) {
                c.addOrder(Order.asc(page.getOrderBy()));
            } else {
                c.addOrder(Order.desc(page.getOrderBy()));
            }
        }
 
        page.setData(c.list());
 
        return page;
    }
 
 
    /**
     * �����Բ��Ҷ����б�.
     */
    public List<T> findByProperty(String propertyName, Object value) {
        Assert.hasText(propertyName);
        return createCriteria(Restrictions.eq(propertyName, value)).list();
    }
 
    /**
     * �����Բ���Ψһ����.
     */
    public T findUniqueByProperty(String propertyName, Object value) {
        Assert.hasText(propertyName);
        return (T) createCriteria(Restrictions.eq(propertyName, value)).uniqueResult();
    }
 
    /**
     * ���ݲ�ѯ����������б���Query����,�����ɽ��и��ദ��,��������.
     */
    public Query createQuery(String queryString, Object... values) {
        Assert.hasText(queryString);
        Query queryObject = getSession().createQuery(queryString);
        if (values != null) {
            for (int i = 0; i < values.length; i++) {
                queryObject.setParameter(i, values[i]);
            }
        }
        return queryObject;
    }
 
    /**
     * ����Criterion��������Criteria,�����ɽ��и��ദ��,��������.
     */
    public Criteria createCriteria(Criterion... criterions) {
        Criteria criteria = getSession().createCriteria(entityClass);
        for (Criterion c : criterions) {
            criteria.add(c);
        }
        return criteria;
    }
 
    /**
     * �ж϶��������ֵ�����ݿ����Ƿ�Ψһ.
     * <p/>
     * ���޸Ķ�����龰��,����������޸ĵ�ֵ(value)��������ԭֵ(orgValue)�����Ƚ�.
     * ����orgValue����Ʋ����ڴ�ҳ���Ϸ���Ajax�ж�����ĳ���.
     * ������ҪSS2�������Զ���ID��Ϊ��3��������isUnique����.
     */
    public boolean isPropertyUnique(String propertyName, Object newValue, Object orgValue) {
        if (newValue == null || newValue.equals(orgValue))
            return true;
 
        Object object = findUniqueByProperty(propertyName, newValue);
        return (object == null);
    }
 
    /**
     * ͨ��count��ѯ��ñ��β�ѯ���ܻ�õĶ�������.
     *
     * @return page�����е�totalCount���Խ���ֵ.
     */
    protected long countQueryResult(PageUtils<T> page, Criteria c) {
        CriteriaImpl impl = (CriteriaImpl) c;
 
        // �Ȱ�Projection��ResultTransformer��OrderByȡ����,������ߺ���ִ��Count����
        Projection projection = impl.getProjection();
        ResultTransformer transformer = impl.getResultTransformer();
 
        List<CriteriaImpl.OrderEntry> orderEntries = null;
        try {
            orderEntries = (List<CriteriaImpl.OrderEntry>) BeanUtils.getFieldValue(impl, "orderEntries");
            BeanUtils.setFieldValue(impl, "orderEntries", new ArrayList<CriteriaImpl.OrderEntry>());
        } catch (Exception e) {
            logger.error("�������׳����쳣:{}", e.getMessage());
        }
 
        // ִ��Count��ѯ
        long totalCount = (Long) c.setProjection(Projections.rowCount()).uniqueResult();
        if (totalCount < 1)
            return -1;
 
        // ��֮ǰ��Projection��OrderBy�����������ȥ
        c.setProjection(projection);
 
        if (projection == null) {
            c.setResultTransformer(CriteriaSpecification.ROOT_ENTITY);
        }
        if (transformer != null) {
            c.setResultTransformer(transformer);
        }
 
        try {
            BeanUtils.setFieldValue(impl, "orderEntries", orderEntries);
        } catch (Exception e) {
            logger.error("�������׳����쳣:{}", e.getMessage());
        }
 
        return totalCount;
    }
}
org.springframework.beans.factory.BeanCreationException: Error creating bean with name 'com.mx.hr.dao.impl.ApplyDao' defined in file [D:\workspace\hr\build\classes\com\mx\hr\dao\impl\ApplyDao.class]: Instantiation of bean failed; nested exception is org.springframework.beans.BeanInstantiationException: Could not instantiate bean class [com.mx.hr.dao.impl.ApplyDao]: No default constructor found; nested exception is java.lang.NoSuchMethodException: com.mx.hr.dao.impl.ApplyDao.<init>()
	at org.springframework.beans.factory.support.AbstractAutowireCapableBeanFactory.instantiateBean(AbstractAutowireCapableBeanFactory.java:1007)
	at org.springframework.beans.factory.support.AbstractAutowireCapableBeanFactory.createBeanInstance(AbstractAutowireCapableBeanFactory.java:953)
	at org.springframework.beans.factory.support.AbstractAutowireCapableBeanFactory.doCreateBean(AbstractAutowireCapableBeanFactory.java:487)
	at org.springframework.beans.factory.support.AbstractAutowireCapableBeanFactory.createBean(AbstractAutowireCapableBeanFactory.java:458)
	at org.springframework.beans.factory.support.AbstractBeanFactory$1.getObject(AbstractBeanFactory.java:295)
	at org.springframework.beans.factory.support.DefaultSingletonBeanRegistry.getSingleton(DefaultSingletonBeanRegistry.java:223)
	at org.springframework.beans.factory.support.AbstractBeanFactory.doGetBean(AbstractBeanFactory.java:292)
	at org.springframework.beans.factory.support.AbstractBeanFactory.getBean(AbstractBeanFactory.java:194)
	at org.springframework.beans.factory.support.DefaultListableBeanFactory.preInstantiateSingletons(DefaultListableBeanFactory.java:628)
	at org.springframework.context.support.AbstractApplicationContext.finishBeanFactoryInitialization(AbstractApplicationContext.java:932)
	at org.springframework.context.support.AbstractApplicationContext.refresh(AbstractApplicationContext.java:479)
	at org.springframework.context.support.ClassPathXmlApplicationContext.<init>(ClassPathXmlApplicationContext.java:139)
	at org.springframework.context.support.ClassPathXmlApplicationContext.<init>(ClassPathXmlApplicationContext.java:83)
	at com.mx.hr.test._Test.test1(_Test.java:24)
	at sun.reflect.NativeMethodAccessorImpl.invoke0(Native Method)
	at sun.reflect.NativeMethodAccessorImpl.invoke(NativeMethodAccessorImpl.java:57)
	at sun.reflect.DelegatingMethodAccessorImpl.invoke(DelegatingMethodAccessorImpl.java:43)
	at java.lang.reflect.Method.invoke(Method.java:606)
	at org.junit.runners.model.FrameworkMethod$1.runReflectiveCall(FrameworkMethod.java:47)
	at org.junit.internal.runners.model.ReflectiveCallable.run(ReflectiveCallable.java:12)
	at org.junit.runners.model.FrameworkMethod.invokeExplosively(FrameworkMethod.java:44)
	at org.junit.internal.runners.statements.InvokeMethod.evaluate(InvokeMethod.java:17)
	at org.junit.runners.ParentRunner.runLeaf(ParentRunner.java:271)
	at org.junit.runners.BlockJUnit4ClassRunner.runChild(BlockJUnit4ClassRunner.java:70)
	at org.junit.runners.BlockJUnit4ClassRunner.runChild(BlockJUnit4ClassRunner.java:50)
	at org.junit.runners.ParentRunner$3.run(ParentRunner.java:238)
	at org.junit.runners.ParentRunner$1.schedule(ParentRunner.java:63)
	at org.junit.runners.ParentRunner.runChildren(ParentRunner.java:236)
	at org.junit.runners.ParentRunner.access$000(ParentRunner.java:53)
	at org.junit.runners.ParentRunner$2.evaluate(ParentRunner.java:229)
	at org.junit.runners.ParentRunner.run(ParentRunner.java:309)
	at org.eclipse.jdt.internal.junit4.runner.JUnit4TestReference.run(JUnit4TestReference.java:50)
	at org.eclipse.jdt.internal.junit.runner.TestExecution.run(TestExecution.java:38)
	at org.eclipse.jdt.internal.junit.runner.RemoteTestRunner.runTests(RemoteTestRunner.java:459)
	at org.eclipse.jdt.internal.junit.runner.RemoteTestRunner.runTests(RemoteTestRunner.java:675)
	at org.eclipse.jdt.internal.junit.runner.RemoteTestRunner.run(RemoteTestRunner.java:382)
	at org.eclipse.jdt.internal.junit.runner.RemoteTestRunner.main(RemoteTestRunner.java:192)
Caused by: org.springframework.beans.BeanInstantiationException: Could not instantiate bean class [com.mx.hr.dao.impl.ApplyDao]: No default constructor found; nested exception is java.lang.NoSuchMethodException: com.mx.hr.dao.impl.ApplyDao.<init>()
	at org.springframework.beans.factory.support.SimpleInstantiationStrategy.instantiate(SimpleInstantiationStrategy.java:83)
	at org.springframework.beans.factory.support.AbstractAutowireCapableBeanFactory.instantiateBean(AbstractAutowireCapableBeanFactory.java:1000)
	... 36 more
Caused by: java.lang.NoSuchMethodException: com.mx.hr.dao.impl.ApplyDao.<init>()
	at java.lang.Class.getConstructor0(Class.java:2849)
	at java.lang.Class.getDeclaredConstructor(Class.java:2053)
	at org.springframework.beans.factory.support.SimpleInstantiationStrategy.instantiate(SimpleInstantiationStrategy.java:78)
	... 37 more

