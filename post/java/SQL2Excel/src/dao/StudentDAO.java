package dao;
// default package

import java.util.List;


import model.Student;

import org.apache.commons.logging.Log;
import org.apache.commons.logging.LogFactory;
import org.hibernate.LockMode;
import org.hibernate.Query;
import org.hibernate.criterion.Example;

/**
 	* A data access object (DAO) providing persistence and search support for Student entities.
 			* Transaction control of the save(), update() and delete() operations 
		can directly support Spring container-managed transactions or they can be augmented	to handle user-managed Spring transactions. 
		Each of these methods provides additional information for how to configure it for the desired type of transaction control. 	
	 * @see .Student
  * @author MyEclipse Persistence Tools 
 */
public class StudentDAO extends BaseHibernateDAO  {
		 private static final Log log = LogFactory.getLog(StudentDAO.class);
		//property constants
	public static final String NAME = "name";
	public static final String GENDER = "gender";
	public static final String AGE = "age";
	public static final String ADDR = "addr";



    
    public void save(Student transientInstance) {
        log.debug("saving Student instance");
        try {
            getSession().save(transientInstance);
            log.debug("save successful");
        } catch (RuntimeException re) {
            log.error("save failed", re);
            throw re;
        }
    }
    
	public void delete(Student persistentInstance) {
        log.debug("deleting Student instance");
        try {
            getSession().delete(persistentInstance);
            log.debug("delete successful");
        } catch (RuntimeException re) {
            log.error("delete failed", re);
            throw re;
        }
    }
    
    public Student findById( java.lang.String id) {
        log.debug("getting Student instance with id: " + id);
        try {
            Student instance = (Student) getSession()
                    .get("Student", id);
            return instance;
        } catch (RuntimeException re) {
            log.error("get failed", re);
            throw re;
        }
    }
    
    
    public List findByExample(Student instance) {
        log.debug("finding Student instance by example");
        try {
            List results = getSession()
                    .createCriteria("Student")
                    .add(Example.create(instance))
            .list();
            log.debug("find by example successful, result size: " + results.size());
            return results;
        } catch (RuntimeException re) {
            log.error("find by example failed", re);
            throw re;
        }
    }    
    
    public List findByProperty(String propertyName, Object value) {
      log.debug("finding Student instance with property: " + propertyName
            + ", value: " + value);
      try {
         String queryString = "from Student as model where model." 
         						+ propertyName + "= ?";
         Query queryObject = getSession().createQuery(queryString);
		 queryObject.setParameter(0, value);
		 return queryObject.list();
      } catch (RuntimeException re) {
         log.error("find by property name failed", re);
         throw re;
      }
	}

	public List findByName(Object name
	) {
		return findByProperty(NAME, name
		);
	}
	
	public List findByGender(Object gender
	) {
		return findByProperty(GENDER, gender
		);
	}
	
	public List findByAge(Object age
	) {
		return findByProperty(AGE, age
		);
	}
	
	public List findByAddr(Object addr
	) {
		return findByProperty(ADDR, addr
		);
	}
	

	public List findAll() {
		log.debug("finding all Student instances");
		try {
			String queryString = "from Student";
	         Query queryObject = getSession().createQuery(queryString);
			 return queryObject.list();
		} catch (RuntimeException re) {
			log.error("find all failed", re);
			throw re;
		}
	}
	
    public Student merge(Student detachedInstance) {
        log.debug("merging Student instance");
        try {
            Student result = (Student) getSession()
                    .merge(detachedInstance);
            log.debug("merge successful");
            return result;
        } catch (RuntimeException re) {
            log.error("merge failed", re);
            throw re;
        }
    }

    public void attachDirty(Student instance) {
        log.debug("attaching dirty Student instance");
        try {
            getSession().saveOrUpdate(instance);
            log.debug("attach successful");
        } catch (RuntimeException re) {
            log.error("attach failed", re);
            throw re;
        }
    }
    
    public void attachClean(Student instance) {
        log.debug("attaching clean Student instance");
        try {
            getSession().lock(instance, LockMode.NONE);
            log.debug("attach successful");
        } catch (RuntimeException re) {
            log.error("attach failed", re);
            throw re;
        }
    }
}