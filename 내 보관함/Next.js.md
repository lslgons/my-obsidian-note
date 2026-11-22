# Next.js

> 출처: [Notion 페이지](https://app.notion.com/p/20aff97ae11380c7b03cc3095bcb1088) · 동기화: 2026-11-22

[App Router: Getting Started](https://nextjs.org/learn/dashboard-app/getting-started)

# 프로젝트 만들기

```bash
npm install -g pnpm

# 대시보드 boilerplate
npx create-next-app@latest nextjs-dashboard --example "https://github.com/vercel/next-learn/tree/main/dashboard/starter-example" --use-pnpm
```

## app 폴더 구조

Next.js 13부터 도입된 App Router는 다음과 같은 폴더 구조를 가집니다:

- **app/** - 모든 라우트와 컴포넌트가 위치하는 루트 폴더
- **app/layout.tsx** - 전체 앱의 공통 레이아웃을 정의하는 루트 레이아웃 파일
- **app/page.tsx** - 앱의 홈페이지를 정의하는 파일
- **app/ui/** - 재사용 가능한 UI 컴포넌트들을 포함하는 폴더
- **app/lib/** - 유틸리티 함수, 데이터 fetching 로직 등을 포함하는 폴더

## 주요 특징

- **서버 컴포넌트:** 기본적으로 모든 컴포넌트는 서버 컴포넌트로 동작
- **중첩 라우팅:** 폴더 구조를 통한 직관적인 라우팅 시스템
- **레이아웃:** 중첩된 레이아웃을 쉽게 구현 가능
- **로딩 UI:** loading.tsx를 통한 suspense 기반 로딩 상태 처리
- **에러 처리:** error.tsx를 통한 자동 에러 바운더리 구현

## 특수 파일

- **layout.tsx** - 공유 UI를 위한 레이아웃 정의
- **page.tsx** - 페이지의 고유 UI 구현
- **loading.tsx** - 로딩 상태를 위한 UI
- **error.tsx** - 에러 상태를 위한 UI
- **not-found.tsx** - 404 에러 페이지

## 실행

```bash
pnpm dev
```

# Style 적용

Tailwind CSS 라이브러리 적용 global.css

```css
@tailwind base;
@tailwind components;
@tailwind utilities;
```

```ts
// CSS 호출
import '@/app/ui/global.css';
```

## Tailwind CSS 특징

[https://tailwindcss.com/](https://tailwindcss.com/)

- **유틸리티 우선 접근방식:** 미리 정의된 클래스를 조합하여 스타일링
- **반응형 디자인:** sm:, md:, lg: 등의 접두사로 쉽게 반응형 구현
- **커스터마이징:** tailwind.config.js를 통해 테마와 스타일 확장 가능
- **개발 생산성:** 별도의 CSS 파일 작성 없이 HTML 내에서 스타일링

## 자주 사용되는 클래스

- **레이아웃:** flex, grid, container, p-{size}, m-{size}
- **타이포그래피:** text-{size}, font-{weight}, text-{color}
- **색상:** bg-{color}, text-{color}, border-{color}
- **효과:** hover:, focus:, active: 등의 상태 수정자
