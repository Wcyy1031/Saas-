package org.example.day1demo.common;

import lombok.Data;

@Data
public class BizException extends RuntimeException {
    private Integer code;
    public BizException(Integer code, String msg) {
        super(msg);
        this.code = code;
    }
}
