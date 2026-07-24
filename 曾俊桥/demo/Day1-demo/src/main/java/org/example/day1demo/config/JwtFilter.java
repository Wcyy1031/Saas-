package org.example.day1demo.config;

import io.jsonwebtoken.Claims;
import io.jsonwebtoken.Jwts;
import io.jsonwebtoken.security.Keys;
import jakarta.servlet.FilterChain;
import jakarta.servlet.ServletException;
import jakarta.servlet.http.HttpServletRequest;
import jakarta.servlet.http.HttpServletResponse;
import org.example.day1demo.common.BaseContext;
import org.springframework.core.annotation.Order;
import org.springframework.stereotype.Component;
import org.springframework.web.filter.OncePerRequestFilter;

import javax.crypto.SecretKey;
import java.io.IOException;

@Component
@Order(1)
public class JwtFilter extends OncePerRequestFilter {
    // 测试固定密钥，生产需加密存放
    private final String SECRET_KEY = "test-secret-key-1234567890";

    @Override
    protected void doFilterInternal(HttpServletRequest request,
                                    HttpServletResponse response,
                                    FilterChain filterChain) throws ServletException, IOException {
        try {
            String token = request.getHeader("token");
            if (token != null && !token.isEmpty()) {
                SecretKey key = Keys.hmacShaKeyFor(SECRET_KEY.getBytes());
                Claims claims = Jwts.parser()
                        .verifyWith(key)
                        .build()
                        .parseSignedClaims(token)
                        .getPayload();
                Long tenantId = Long.valueOf(claims.get("tenantId").toString());
                BaseContext.setCurrentTenantId(tenantId);
            }
            filterChain.doFilter(request, response);
        } finally {
            // 必须清理，防止线程池复用污染
            BaseContext.removeCurrentTenantId();
        }
    }
}
