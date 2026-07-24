package org.example.day1demo.entity;

import com.baomidou.mybatisplus.annotation.*;
import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.NoArgsConstructor;
import org.example.day1demo.handler.StringListJsonTypeHandler;

import java.time.LocalDateTime;
import java.util.List;

@Data
@Builder
@NoArgsConstructor
@AllArgsConstructor
@TableName(value = "audit_task", autoResultMap = true) // 必须开autoResultMap，TypeHandler才生效
public class AuditTask {

    @TableId(type = IdType.AUTO)
    private Long id;

    private Long tenantId;

    private String status;

    // 任务3：绑定自定义TypeHandler，映射JSON列
    @TableField(typeHandler = StringListJsonTypeHandler.class)
    private List<String> enabledChecks;

    // 乐观锁字段
    @Version
    private Integer version;

    private LocalDateTime createdAt;
}

