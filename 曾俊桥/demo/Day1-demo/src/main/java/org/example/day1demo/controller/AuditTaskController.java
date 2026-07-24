package org.example.day1demo.controller;

import com.baomidou.mybatisplus.extension.plugins.pagination.Page;
import org.example.day1demo.common.Result;
import org.example.day1demo.entity.AuditTask;

import org.example.day1demo.service.AuditTaskService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.*;

import java.time.LocalDateTime;

@RestController
@RequestMapping("/api/tasks")
public class AuditTaskController {

    @Autowired
    private AuditTaskService auditTaskService;

    // 新增任务（验证CRUD + TypeHandler）
    @PostMapping
    public Result<AuditTask> createTask(@RequestBody AuditTask task) {
        return Result.success(auditTaskService.createTask(task));
    }

    // 分页查询（验证分页 + LambdaQueryWrapper）
    @GetMapping("/page")
    public Result<Page<AuditTask>> pageTasks(
            @RequestParam(defaultValue = "1") int pageNum,
            @RequestParam(defaultValue = "10") int pageSize,
            @RequestParam(required = false) String status,
            @RequestParam(required = false) LocalDateTime startTime) {
        return Result.success(auditTaskService.pageTasks(pageNum, pageSize, status, startTime));
    }

    // 任务1：测试事务失效
    @PostMapping("/test-trans-error")
    public Result<Void> testTransError() {
        auditTaskService.testTransactionError();
        return Result.success(null);
    }

    // 任务1：测试事务修复
    @PostMapping("/test-trans-fix")
    public Result<Void> testTransFix() {
        auditTaskService.testTransactionFix();
        return Result.success(null);
    }
}
