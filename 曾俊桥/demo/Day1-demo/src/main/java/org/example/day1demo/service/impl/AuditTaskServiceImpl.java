package org.example.day1demo.service.impl;

import com.baomidou.mybatisplus.core.conditions.query.LambdaQueryWrapper;
import com.baomidou.mybatisplus.extension.plugins.pagination.Page;
import org.example.day1demo.entity.AuditTask;
import org.example.day1demo.mapper.AuditTaskMapper;
import org.example.day1demo.service.AuditTaskService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.context.annotation.Lazy;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;
import org.springframework.util.StringUtils;

import java.time.LocalDateTime;

@Service
public class AuditTaskServiceImpl implements AuditTaskService {

    @Autowired
    private AuditTaskMapper taskMapper;

    // 注入自身代理对象，用于修复事务失效
    @Autowired
    @Lazy
    private AuditTaskService self;

    // ==================== 任务2：CRUD ====================
    @Override
    public AuditTask createTask(AuditTask task) {
        task.setStatus("PENDING");
        task.setCreatedAt(LocalDateTime.now());
        taskMapper.insert(task); // 插入后id自动回填
        return task;
    }

    // ==================== 任务2：分页 + LambdaQueryWrapper ====================
    @Override
    public Page<AuditTask> pageTasks(int pageNum, int pageSize, String status, LocalDateTime startTime) {
        // 1. 构建分页对象
        Page<AuditTask> page = new Page<>(pageNum, pageSize);

        // 2. 构建类型安全的动态查询条件
        LambdaQueryWrapper<AuditTask> lqw = new LambdaQueryWrapper<>();
        lqw.eq(StringUtils.hasText(status), AuditTask::getStatus, status);
        lqw.gt(startTime != null, AuditTask::getCreatedAt, startTime);
        lqw.orderByDesc(AuditTask::getCreatedAt);

        // 3. 执行分页查询
        return taskMapper.selectPage(page, lqw);
    }

    // ==================== 任务1：事务失效复现 ====================
    @Override
    public void testTransactionError() {
        // this调用原始对象，绕过代理，@Transactional不生效
        this.saveTaskWithException();
    }

    // ==================== 任务1：事务修复 ====================
    @Override
    public void testTransactionFix() {
        // self调用代理对象，触发AOP切面，事务正常生效
        self.saveTaskWithException();
    }

    // 带事务注解的方法
    @Transactional(rollbackFor = Exception.class)
    public void saveTaskWithException() {
        AuditTask task = AuditTask.builder()
                .tenantId(1L)
                .status("PENDING")
                .build();
        taskMapper.insert(task);
        // 模拟异常，预期触发回滚
        throw new RuntimeException("模拟业务异常");
    }
}
