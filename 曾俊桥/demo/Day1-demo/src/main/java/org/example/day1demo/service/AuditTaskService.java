package org.example.day1demo.service;

import com.baomidou.mybatisplus.extension.plugins.pagination.Page;
import org.example.day1demo.entity.AuditTask;

import java.time.LocalDateTime;

public interface AuditTaskService {
    // 任务2：新增任务（CRUD）
    AuditTask createTask(AuditTask task);

    // 任务2：分页查询 + Lambda动态条件
    Page<AuditTask> pageTasks(int pageNum, int pageSize, String status, LocalDateTime startTime);

    // 任务1：事务失效复现（错误写法）
    void testTransactionError();

    // 任务1：事务修复（正确写法）
    void testTransactionFix();

    void saveTaskWithException();
}
