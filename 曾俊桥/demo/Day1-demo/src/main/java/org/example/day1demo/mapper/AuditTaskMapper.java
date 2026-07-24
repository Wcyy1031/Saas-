package org.example.day1demo.mapper;

import com.baomidou.mybatisplus.core.mapper.BaseMapper;
import org.apache.ibatis.annotations.Mapper;
import org.example.day1demo.entity.AuditTask;

@Mapper
public interface AuditTaskMapper extends BaseMapper<AuditTask> {
    // 基础CRUD全部由BaseMapper提供，无需手写
}
