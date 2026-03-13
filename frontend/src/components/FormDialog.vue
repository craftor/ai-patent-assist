<template>
  <el-dialog
    v-model="dialogVisible"
    :title="title"
    :width="width"
    :close-on-click-modal="closeOnClickModal"
    :close-on-press-escape="closeOnPressEscape"
    :show-close="showClose"
    :align-center="alignCenter"
    @open="handleOpen"
    @close="handleClose"
    @opened="handleOpened"
    @closed="handleClosed"
  >
    <el-form
      ref="formRef"
      :model="formData"
      :rules="formRules"
      :label-width="labelWidth"
      :label-position="labelPosition"
      :size="size"
      :disabled="formDisabled"
      @submit.prevent
    >
      <!-- 动态表单字段 -->
      <el-form-item
        v-for="field in fields"
        :key="field.prop"
        :label="field.label"
        :prop="field.prop"
        :required="field.required"
        :rules="field.rules"
        v-bind="field.formItemProps"
      >
        <!-- 文本输入框 -->
        <el-input
          v-if="field.type === 'input' || field.type === 'text'"
          v-model="formData[field.prop]"
          :type="field.inputType || 'text'"
          :placeholder="field.placeholder || `请输入${field.label}`"
          :maxlength="field.maxlength"
          :show-word-limit="field.showWordLimit"
          :rows="field.rows"
          :disabled="field.disabled"
          :readonly="field.readonly"
          :clearable="field.clearable ?? true"
          v-bind="field.props"
        />

        <!-- 多行文本 -->
        <el-input
          v-else-if="field.type === 'textarea'"
          v-model="formData[field.prop]"
          type="textarea"
          :placeholder="field.placeholder || `请输入${field.label}`"
          :maxlength="field.maxlength"
          :show-word-limit="field.showWordLimit"
          :rows="field.rows || 4"
          :disabled="field.disabled"
          :readonly="field.readonly"
          v-bind="field.props"
        />

        <!-- 数字输入框 -->
        <el-input-number
          v-else-if="field.type === 'number'"
          v-model="formData[field.prop]"
          :placeholder="field.placeholder || `请输入${field.label}`"
          :min="field.min"
          :max="field.max"
          :step="field.step || 1"
          :precision="field.precision"
          :disabled="field.disabled"
          :controls="field.controls ?? true"
          v-bind="field.props"
        />

        <!-- 下拉选择框 -->
        <el-select
          v-else-if="field.type === 'select'"
          v-model="formData[field.prop]"
          :placeholder="field.placeholder || `请选择${field.label}`"
          :multiple="field.multiple"
          :clearable="field.clearable ?? true"
          :disabled="field.disabled"
          :filterable="field.filterable"
          :allow-create="field.allowCreate"
          v-bind="field.props"
        >
          <el-option
            v-for="option in field.options"
            :key="option.value"
            :label="option.label"
            :value="option.value"
            :disabled="option.disabled"
          />
        </el-select>

        <!-- 单选框组 -->
        <el-radio-group
          v-else-if="field.type === 'radio'"
          v-model="formData[field.prop]"
          :disabled="field.disabled"
          v-bind="field.props"
        >
          <el-radio
            v-for="option in field.options"
            :key="option.value"
            :label="option.value"
            :disabled="option.disabled"
          >
            {{ option.label }}
          </el-radio>
        </el-radio-group>

        <!-- 复选框组 -->
        <el-checkbox-group
          v-else-if="field.type === 'checkbox'"
          v-model="formData[field.prop]"
          :disabled="field.disabled"
          v-bind="field.props"
        >
          <el-checkbox
            v-for="option in field.options"
            :key="option.value"
            :label="option.value"
            :disabled="option.disabled"
          >
            {{ option.label }}
          </el-checkbox>
        </el-checkbox-group>

        <!-- 日期选择器 -->
        <el-date-picker
          v-else-if="field.type === 'date'"
          v-model="formData[field.prop]"
          :type="field.dateType || 'date'"
          :placeholder="field.placeholder || `请选择${field.label}`"
          :disabled="field.disabled"
          :clearable="field.clearable ?? true"
          :format="field.format"
          :value-format="field.valueFormat"
          v-bind="field.props"
        />

        <!-- 日期时间范围选择器 -->
        <el-date-picker
          v-else-if="field.type === 'datetime'"
          v-model="formData[field.prop]"
          type="datetime"
          :placeholder="field.placeholder || `请选择${field.label}`"
          :disabled="field.disabled"
          :clearable="field.clearable ?? true"
          :format="field.format"
          :value-format="field.valueFormat"
          v-bind="field.props"
        />

        <!-- 开关 -->
        <el-switch
          v-else-if="field.type === 'switch'"
          v-model="formData[field.prop]"
          :disabled="field.disabled"
          :active-text="field.activeText"
          :inactive-text="field.inactiveText"
          v-bind="field.props"
        />

        <!-- 自定义插槽 -->
        <slot
          v-else-if="field.type === 'slot'"
          :name="field.prop"
          :model="formData[field.prop]"
          :field="field"
        />
      </el-form-item>

      <!-- 自定义内容 -->
      <slot name="custom" />
    </el-form>

    <template #footer>
      <span v-if="showCancel" class="dialog-footer">
        <el-button
          :disabled="loading"
          @click="handleCancel"
        >
          {{ cancelText }}
        </el-button>
      </span>
      <el-button
        v-if="showReset"
        :disabled="loading || formDisabled"
        @click="handleReset"
      >
        重置
      </el-button>
      <el-button
        type="primary"
        :loading="loading"
        :disabled="formDisabled"
        @click="handleSubmit"
      >
        {{ confirmText }}
      </el-button>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import type { PropType, FormInstance, FormRules } from 'vue'
import type { SelectOptionProps } from 'element-plus'

export interface FormField {
  prop: string
  label: string
  type: 'input' | 'textarea' | 'number' | 'select' | 'radio' | 'checkbox' | 'date' | 'datetime' | 'switch' | 'slot'
  inputType?: 'text' | 'password' | 'email' | 'url' | 'tel'
  dateType?: 'year' | 'month' | 'date' | 'dates' | 'week' | 'datetime' | 'datetimerange' | 'daterange' | 'monthrange'
  placeholder?: string
  required?: boolean
  rules?: FormRules[string]
  options?: SelectOptionProps[]
  disabled?: boolean
  readonly?: boolean
  clearable?: boolean
  maxlength?: number
  showWordLimit?: boolean
  rows?: number
  min?: number
  max?: number
  step?: number
  precision?: number
  multiple?: boolean
  filterable?: boolean
  allowCreate?: boolean
  format?: string
  valueFormat?: string
  activeText?: string
  inactiveText?: string
  props?: any
  formItemProps?: any
}

const props = defineProps({
  // 对话框控制
  modelValue: {
    type: Boolean,
    default: false,
  },
  title: {
    type: String,
    default: '',
  },
  width: {
    type: [Number, String],
    default: '500px',
  },
  closeOnClickModal: {
    type: Boolean,
    default: false,
  },
  closeOnPressEscape: {
    type: Boolean,
    default: true,
  },
  showClose: {
    type: Boolean,
    default: true,
  },
  alignCenter: {
    type: Boolean,
    default: true,
  },
  // 表单数据
  formData: {
    type: Object as PropType<Record<string, any>>,
    required: true,
  },
  // 表单字段配置
  fields: {
    type: Array as PropType<FormField[]>,
    required: true,
  },
  // 表单验证规则
  formRules: {
    type: Object as PropType<FormRules>,
    default: () => ({}),
  },
  // 表单配置
  labelWidth: {
    type: String,
    default: '80px',
  },
  labelPosition: {
    type: String as PropType<'left' | 'right' | 'top'>,
    default: 'right',
  },
  size: {
    type: String as PropType<'default' | 'small' | 'large'>,
    default: 'default',
  },
  formDisabled: {
    type: Boolean,
    default: false,
  },
  // 按钮配置
  loading: {
    type: Boolean,
    default: false,
  },
  showCancel: {
    type: Boolean,
    default: true,
  },
  showReset: {
    type: Boolean,
    default: false,
  },
  cancelText: {
    type: String,
    default: '取消',
  },
  confirmText: {
    type: String,
    default: '确定',
  },
})

const emit = defineEmits<{
  (e: 'update:modelValue', value: boolean): void
  (e: 'open'): void
  (e: 'close'): void
  (e: 'opened'): void
  (e: 'closed'): void
  (e: 'submit', data: any): void
  (e: 'cancel'): void
  (e: 'reset'): void
  (e: 'validate-success'): void
  (e: 'validate-error'): void
}>()

const formRef = ref<FormInstance>()

const dialogVisible = computed({
  get: () => props.modelValue,
  set: (value) => emit('update:modelValue', value),
})

const handleOpen = () => emit('open')
const handleClose = () => {
  formRef.value?.resetFields()
  emit('close')
}
const handleOpened = () => emit('opened')
const handleClosed = () => emit('closed')

const handleCancel = () => {
  dialogVisible.value = false
  emit('cancel')
}

const handleReset = () => {
  formRef.value?.resetFields()
  emit('reset')
}

const handleSubmit = async () => {
  if (!formRef.value) return

  try {
    await formRef.value.validate()
    emit('submit', props.formData)
  } catch (error) {
    emit('validate-error', error)
  }
}

// 暴露方法给父组件
const validate = async () => {
  if (!formRef.value) return true
  try {
    await formRef.value.validate()
    return true
  } catch {
    return false
  }
}

const resetFields = () => {
  formRef.value?.resetFields()
}

const validateField = (props: string | string[]) => {
  formRef.value?.validateField(props)
}

const clearValidate = () => {
  formRef.value?.clearValidate()
}

defineExpose({
  validate,
  resetFields,
  validateField,
  clearValidate,
  formRef,
})
</script>

<style scoped>
.dialog-footer {
  display: inline-flex;
  gap: 8px;
}

:deep(.el-dialog__body) {
  padding-top: 20px;
}

:deep(.el-form-item) {
  margin-bottom: 20px;
}

:deep(.el-form-item:last-child) {
  margin-bottom: 0;
}

@media screen and (max-width: 768px) {
  :deep(.el-dialog) {
    width: 90% !important;
    max-width: 500px;
  }

  :deep(.el-form-item__label) {
    font-size: 14px;
  }
}
</style>
