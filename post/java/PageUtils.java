/*
 * Copyright (c) 2012 DongXiangli.All rights reserved.
 */
 
package com.codeworker.utils.dao;
 
import java.io.Serializable;
import java.util.List;
 
/**
 * User: DongXiangli
 * Date: 12-12-23
 * Time: 下午10:48
 */
public class PageUtils<T> implements Serializable {
    public static final String ASC = "asc";
    public static final String DESC = "desc";
 
    protected int pageNo = 1;                //当前页号
    protected int pageSize = -1;             //页内记录数
    protected String orderBy = null;         //排序字段，基本没有使用
    protected String order = ASC;            //排序方向
    protected boolean autoCount = false;     //自动计算记录数
 
    private List<T> data = null;             //数据
    private long totalCount = -1;            //总记录数
 
    public PageUtils() {
        this(1, 20);
    }
 
    public PageUtils(int pageIndex) {
        this(pageIndex, 20);
    }
 
    public PageUtils(int pageIndex, int pageSize) {
        if (pageIndex < 1) pageIndex = 1;
        if (pageSize < 1) pageSize = 15;
        this.pageNo = pageIndex;
        this.pageSize = pageSize;
    }
 
    public PageUtils(int pageSize, boolean autoCount) {
        if (pageSize <= 0) {
            pageSize = 20;
        }
        this.pageSize = pageSize;
        this.autoCount = autoCount;
    }
 
    /**
     * 获得每页的记录数量,无默认值.
     */
    public int getPageSize() {
        return pageSize;
    }
 
    public void setPageSize(int pageSize) {
        this.pageSize = pageSize;
    }
 
    /**
     * 是否已设置每页的记录数量.
     */
    public boolean isPageSizeSetted() {
        return pageSize > -1;
    }
 
    /**
     * 获得当前页的页号,序号从1开始,默认为1.
     */
    public int getPageNo() {
        return pageNo;
    }
 
    public void setPageNo(int pageNo) {
        this.pageNo = pageNo;
    }
 
    /**
     * 根据pageNo和pageSize计算当前页第一条记录在总结果集中的位置,序号从0开始.
     */
    public int getFirst() {
        if (pageNo < 1 || pageSize < 1)
            return -1;
        else
            return ((pageNo - 1) * pageSize);
    }
 
    /**
     * 是否已设置第一条记录记录在总结果集中的位置.
     */
    public boolean isFirstSetted() {
        return (pageNo > 0 && pageSize > 0);
    }
 
    /**
     * 获得排序字段,无默认值.
     */
    public String getOrderBy() {
        return orderBy;
    }
 
    public void setOrderBy(String orderBy) {
        this.orderBy = orderBy;
    }
 
    /**
     * 是否已设置排序字段.
     */
    public boolean isOrderBySetted() {
        if (orderBy != null) {
            orderBy = orderBy.trim();
            return !orderBy.isEmpty();
        }
        return false;
    }
 
    /**
     * 获得排序方向,默认为asc.
     */
    public String getOrder() {
        return order;
    }
 
    /**
     * 设置排序方式向.
     *
     * @param order 可选值为desc或asc.
     */
    public void setOrder(String order) {
        if (ASC.equalsIgnoreCase(order) || DESC.equalsIgnoreCase(order)) {
            this.order = order.toLowerCase();
        } else
            throw new IllegalArgumentException("order should be 'desc' or 'asc'");
    }
 
    /**
     * 是否自动获取总页数,默认为false.
     * 注意本属性仅于query by Criteria时有效,query by HQL时本属性无效.
     */
    public boolean isAutoCount() {
        return autoCount;
    }
 
    public void setAutoCount(boolean autoCount) {
        this.autoCount = autoCount;
    }
 
    /**
     * 取得倒转的排序方向
     */
    public String getInverseOrder() {
        if (order.endsWith(DESC))
            return ASC;
        else
            return DESC;
    }
 
    /**
     * 页内的数据列表.
     */
    public List<T> getData() {
        return this.data;
    }
 
    public void setData(List<T> data) {
        this.data = data;
    }
 
    /**
     * 总记录数.
     */
    public long getTotalCount() {
        return totalCount;
    }
 
    public void setTotalCount(long totalCount) {
        this.totalCount = totalCount;
    }
 
    /**
     * 计算总页数.
     */
    public long getTotalPages() {
        if (totalCount == -1)
            return 1;
 
        long count = totalCount / pageSize;
        if (totalCount % pageSize > 0) {
            count++;
        }
        return count;
    }
 
    /**
     * 是否还有下一页.
     */
    public boolean isHasNext() {
        return (pageNo + 1 <= getTotalPages());
    }
 
    /**
     * 返回下页的页号,序号从1开始.
     */
    public int getNextPage() {
        if (isHasNext())
            return pageNo + 1;
        else
            return pageNo;
    }
 
    /**
     * 是否还有上一页.
     */
    public boolean isHasPre() {
        return (pageNo - 1 >= 1);
    }
 
    /**
     * 返回上页的页号,序号从1开始.
     */
    public int getPrePage() {
        if (isHasPre())
            return pageNo - 1;
        else
            return pageNo;
    }
 
}