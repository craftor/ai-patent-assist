<template>
  <div class="data-table">
    <el-card>
      <!-- 表格工具栏 -->
      <div v-if="showToolbar" class="table-toolbar">
        <div class="toolbar-left">
          <slot name="toolbar-left"></slot>
        </div>
        <div class="toolbar-right">
          <slot name="toolbar-right">
            <el-input
              v-if="searchable"
              v-model="searchQuery"
              placeholder="搜索..."
              clearable
              style="width: 200px"
              @input="handleSearch"
            >
              <template #prefix>
                <el-icon><Search /></el-icon>
              </template>
            </el-input>
            <el-button
              v-if="refreshable"
              :icon="Refresh"
              circle
              @click="handleRefresh"
            />
          </slot>
        </div>
      </div>

      <!-- 数据表格 -->
      <el-table
        :data="data"
        :loading="loading"
        :stripe="stripe"
        :border="border"
        :row-key="rowKey"
        @sort-change="handleSortChange"
        @selection-change="handleSelectionChange"
        style="width: 100%"
      >
        <!-- 选择列 -->
        <el-table-column
          v-if="selectable"
          type="selection"
          :reserve-selection="reserveSelection"
          width="55"
        />

        <!-- 索引列 -->
        <el-table-column
          v-if="showIndex"
          type="index"
          :index="indexMethod"
          :label="indexLabel"
          width="60"
          align="center"
        />

        <!-- 动态列 -->
        <slot>
          <el-table-column
            v-for="column in columns"
            :key="column.prop"
            :prop="column.prop"
            :label="column.label"
            :width="column.width"
            :min-width="column.minWidth"
            :sortable="column.sortable"
            :align="column.align || 'left'"
          >
            <template #default="scope" v-if="column.slotName">
              <slot :name="column.slotName" :row="scope.row" :index="scope.$index" />
            </template>
          </el-table-column>
        </slot>

        <!-- 操作列 -->
        <el-table-column
          v-if="showActions"
          :label="actionsLabel"
          :width="actionsWidth"
          :fixed="actionsFixed"
          align="center"
        >
          <template #default="{ row, $index }">
            <slot name="actions" :row="row" :index="$index">
              <el-button
                v-if="showEdit"
                type="primary"
                link
                @click="handleEdit(row)"
              >
                编辑
              </el-button>
              <el-button
                v-if="showDelete"
                type="danger"
                link
                @click="handleDelete(row)"
              >
                删除
              </el-button>
            </slot>
          </template>
        </el-table-column>
      </el-table>

      <!-- 分页 -->
      <div v-if="pagination" class="table-pagination">
        <el-pagination
          v-model:current-page="currentPage"
          v-model:page-size="pageSize"
          :page-sizes="pageSizes"
          :total="total"
          :layout="paginationLayout"
          @size-change="handleSizeChange"
          @current-change="handlePageChange"
        />
      </div>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { Search, Refresh } from '@element-plus/icons-vue'
import type { PropType } from 'vue'

export interface TableColumn {
  prop: string
  label: string
  width?: number | string
  minWidth?: number | string
  sortable?: boolean | 'custom'
  align?: 'left' | 'center' | 'right'
  slotName?: string
}

interface Pagination {
  currentPage: number
  pageSize: number
  total: number
}

const props = defineProps({
  // 数据
  data: {
    type: Array as PropType<any[]>,
    required: true,
  },
  // 列配置
  columns: {
    type: Array as PropType<TableColumn[]>,
    default: () => [],
  },
  // 加载状态
  loading: {
    type: Boolean,
    default: false,
  },
  // 斑马纹
  stripe: {
    type: Boolean,
    default: true,
  },
  // 边框
  border: {
    type: Boolean,
    default: false,
  },
  // 行键
  rowKey: {
    type: String,
    default: 'id',
  },
  // 工具栏
  showToolbar: {
    type: Boolean,
    default: true,
  },
  // 可搜索
  searchable: {
    type: Boolean,
    default: true,
  },
  // 可刷新
  refreshable: {
    type: Boolean,
    default: true,
  },
  // 可选择
  selectable: {
    type: Boolean,
    default: false,
  },
  reserveSelection: {
    type: Boolean,
    default: false,
  },
  // 显示索引
  showIndex: {
    type: Boolean,
    default: false,
  },
  indexLabel: {
    type: String,
    default: '序号',
  },
  // 操作列
  showActions: {
    type: Boolean,
    default: true,
  },
  actionsLabel: {
    type: String,
    default: '操作',
  },
  actionsWidth: {
    type: [Number, String],
    default: 150,
  },
  actionsFixed: {
    type: [Boolean, String] as PropType<boolean | 'left' | 'right'>,
    default: 'right',
  },
  showEdit: {
    type: Boolean,
    default: true,
  },
  showDelete: {
    type: Boolean,
    default: true,
  },
  // 分页
  pagination: {
    type: Object as PropType<Pagination | null>,
    default: null,
  },
  pageSizes: {
    type: Array as PropType<number[]>,
    default: () => [10, 20, 50, 100],
  },
  paginationLayout: {
    type: String,
    default: 'total, sizes, prev, pager, next, jumper',
  },
})

const emit = defineEmits<{
  (e: 'search', value: string): void
  (e: 'refresh'): void
  (e: 'sort', { prop, order }: { prop: string; order: string }): void
  (e: 'selection-change', selection: any[]): void
  (e: 'edit', row: any): void
  (e: 'delete', row: any): void
  (e: 'page-change', { page, pageSize }: { page: number; pageSize: number }): void
  (e: 'size-change', pageSize: number): void
}>()

const searchQuery = ref('')
const currentPage = ref(props.pagination?.currentPage || 1)
const pageSize = ref(props.pagination?.pageSize || 10)
const total = ref(props.pagination?.total || 0)

watch(() => props.pagination, (newVal) => {
  if (newVal) {
    currentPage.value = newVal.currentPage
    pageSize.value = newVal.pageSize
    total.value = newVal.total
  }
}, { deep: true })

const handleSearch = (value: string) => {
  emit('search', value)
}

const handleRefresh = () => {
  emit('refresh')
}

const handleSortChange = ({ prop, order }: { prop: string; order: string }) => {
  emit('sort', { prop, order })
}

const handleSelectionChange = (selection: any[]) => {
  emit('selection-change', selection)
}

const handleEdit = (row: any) => {
  emit('edit', row)
}

const handleDelete = (row: any) => {
  emit('delete', row)
}

const handlePageChange = (page: number) => {
  emit('page-change', { page, pageSize: pageSize.value })
}

const handleSizeChange = (size: number) => {
  emit('size-change', size)
}

const indexMethod = (index: number) => index + 1
</script>

<style scoped>
.data-table {
  width: 100%;
}

.table-toolbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
  flex-wrap: wrap;
  gap: 12px;
}

.toolbar-left,
.toolbar-right {
  display: flex;
  align-items: center;
  gap: 8px;
}

.table-pagination {
  display: flex;
  justify-content: flex-end;
  margin-top: 16px;
}

@media screen and (max-width: 768px) {
  .table-toolbar {
    flex-direction: column;
    align-items: stretch;
  }

  .toolbar-left,
  .toolbar-right {
    justify-content: flex-start;
    width: 100%;
  }

  .table-pagination {
    justify-content: center;
  }

  :deep(.el-pagination) {
    --el-pagination-font-size: 12px;
  }
}
</style>
